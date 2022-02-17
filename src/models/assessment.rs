use super::extension::*;
use chrono::NaiveDateTime;
use octocrab::Octocrab;
use serde::{Deserialize, Serialize};
use serde_json::{to_string_pretty, Map, Value};

fn parse_json(content: &String) -> Result<Map<String, Value>, ()> {
    match serde_json::from_str(&content) {
        Ok::<Value, serde_json::Error>(p) => match p.as_object() {
            Some(m) => return Ok(m.clone()),
            None => return Err(()),
        },
        Err(_) => return Err(()),
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignmentAvailability {
    #[serde(skip_serializing_if = "Option::is_none")]
    uids: Option<Vec<String>>,
    credit: u64,
    #[serde(rename = "startDate")]
    start_date: String,
    #[serde(rename = "endDate")]
    end_date: String,
}

pub struct AssessmentInfoFile {
    content: String,
    sha: String,
}

impl AssessmentInfoFile {
    pub fn get_sha(&self) -> &String {
        &self.sha
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }

    pub async fn get(
        owner: &String,
        repo: &String,
        path: &String,
        github_token: &String,
    ) -> Result<Self, ()> {
        let request = Octocrab::builder()
            .personal_token(github_token.clone())
            .build()
            .unwrap()
            .repos(owner, repo)
            .get_content()
            .path(path)
            .r#ref("master")
            .send()
            .await;

        if request.is_err() {
            return Err(());
        }

        // get the encoded content file from the request
        let item = request.as_ref().unwrap().items.get(0).unwrap();

        // remove the newlines in the formatted base64 string
        let decoded_content: String = item.content.clone().unwrap().split("\n").collect();

        // decode the base64 string to get the raw bytes
        let bytes = base64::decode(decoded_content).unwrap();

        // convert the bytes to a utf8 String
        Ok(AssessmentInfoFile {
            content: String::from_utf8(bytes).unwrap(),
            sha: item.sha.clone(),
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
                    None => None,
                };

                AssignmentAvailability {
                    uids,
                    credit: x.get("credit").unwrap().as_u64().unwrap(),
                    start_date: serde_json::from_value(x.get("startDate").unwrap().clone())
                        .unwrap(),
                    end_date: serde_json::from_value(x.get("endDate").unwrap().clone()).unwrap(),
                }
            })
            .collect::<Vec<AssignmentAvailability>>();

        let date_format: &str = "%Y-%m-%dT%H:%M:%S";
        let start_date: String = availability
            .iter()
            .map(|a| NaiveDateTime::parse_from_str(&a.start_date, date_format).unwrap())
            .min()
            .unwrap()
            .format(date_format)
            .to_string();

        let target_date = extensions.format_date();

        // TODO: add credit field
        for id in extensions.get_netids().into_iter() {
            let mut did_update: bool = false;
            let mut contains_student: bool = false;
            let target_email: String = format!("{}@illinois.edu", id);

            // TODO: implement this logic
            // 1. scan for student
            // 2. scan for target date
            // 3. if student already exists, skip
            // 4. if target date does not exist, create
            // 4. push to target date
            for aa in availability.iter_mut() {
                if aa.uids.is_some() {
                    contains_student |= aa.uids.as_ref().unwrap().contains(&target_email);

                    if contains_student {
                        break;
                    }
                }

                if aa.end_date == target_date {
                    did_update = true;

                    if aa.uids.is_none() {
                        break; // at target date, but it is a generic due date
                    }

                    aa.uids.as_mut().unwrap().push(target_email.clone());
                }
            }

            if !did_update && !contains_student {
                availability.push(AssignmentAvailability {
                    credit: 100,
                    uids: Some(vec![target_email]),
                    start_date: start_date.clone(),
                    end_date: target_date.clone(),
                });
            }
        }

        map.insert(
            "allowAccess".to_string(),
            serde_json::to_value(&availability).unwrap(),
        );

        self.content = to_string_pretty(&map).unwrap();
    }
}
