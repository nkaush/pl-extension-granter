use octocrab::{Octocrab, models::repos::ContentItems};
use serde_json::{Value, Map, to_string_pretty};
use serde::{Deserialize, Serialize};
use super::extension::*;

fn parse_json(content: &String) -> Result<Map<String, Value>, ()> {
    match serde_json::from_str(&content) {
        Ok::<Value, serde_json::Error>(p) => {
            match p.as_object() {
                Some(m) => return Ok(m.clone()),
                None => return Err(())
            }
        },
        Err(_) => return Err(())
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignmentAvailability {
    #[serde(skip_serializing_if = "Option::is_none")]
    uids: Option<Vec<String>>,
    credit: usize,
    #[serde(rename = "startDate")]
    start_date: String,
    #[serde(rename = "endDate")]
    end_date: String
}

pub struct AssessmentInfoFile {
    content: String,
    sha: String
}

impl AssessmentInfoFile {
    pub fn get_sha(&self) -> &String {
        &self.sha
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }

    pub async fn get(owner: &String, repo: &String, path: &String, github_token: &String) -> Result<Self, ()> {        
        let request = Octocrab::builder()
            .personal_token(github_token.clone()).build().unwrap()
            .repos(owner, repo)
            .get_content()
            .path(path)
            .r#ref("master")
            .send()
            .await;
        
        if request.is_err() {
            return Err(());
        }
    
        let content: ContentItems = request.unwrap();
       
        // get the encoded content file from the request
        let item = content.items.get(0).unwrap();
        let encoded_content: String = item.content.clone().unwrap();
        
        // remove the newlines in the formatted base64 string
        let joined: String = encoded_content.split("\n").collect();
    
        // decode the base64 string to get the raw bytes 
        let bytes = base64::decode(joined).unwrap();
    
        // convert the bytes to a utf8 String
        Ok(AssessmentInfoFile {
            content: String::from_utf8(bytes).unwrap(),
            sha: item.sha.clone()
        })
    }

    pub fn grant_extensions(&mut self, extensions: &Extension) {
        let mut map = parse_json(&self.content).unwrap();

        let mut availability: Vec<AssignmentAvailability> = map
            .get("allowAccess")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                let uids = match x.get("uids").clone() {
                    Some(v) => serde_json::from_value(v.clone()).unwrap(),
                    None => None
                };
    
                AssignmentAvailability {
                    uids,
                    credit: serde_json::from_value(x.get("credit").unwrap().clone()).unwrap(),
                    start_date: serde_json::from_value(x.get("startDate").unwrap().clone()).unwrap(),
                    end_date: serde_json::from_value(x.get("endDate").unwrap().clone()).unwrap()
                }
                
            })
            .collect::<Vec<AssignmentAvailability>>();
    
        for id in extensions.get_netids().into_iter() {
            let start_date = availability.get(0).unwrap().start_date.clone();
            let target_date = extensions.format_date();
            let mut did_update: bool = false;
            let mut contains_student: bool = false;
            let target_email: String = format!("{}@illinois.edu", id);
        
            for aa in availability.iter_mut() {
                if aa.uids.is_some() {
                    contains_student |= aa.uids.as_ref().unwrap().contains(&target_email);
                    
                    if contains_student {
                        break;
                    }
                }
                
                if aa.end_date == target_date {
                    if aa.uids.is_none() {
                        aa.uids = Some(Vec::new());
                    }
        
                    aa.uids.as_mut().unwrap().push(target_email.clone());
                    did_update = true;
                }
            }
        
            if !did_update && !contains_student {
                availability.push(AssignmentAvailability {
                    credit: 100,
                    uids: Some(vec![format!("{}@illinois.edu", id)]),
                    start_date,
                    end_date: target_date
                });
            }
        }

        map.insert("allowAccess".to_string(), 
                   serde_json::to_value(&availability).unwrap());

        self.content = to_string_pretty(&map).unwrap();
    }
}
