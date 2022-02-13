use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Extension {
    netids: Vec<String>,
    month: u8,
    day: u8,
    year: u16,
    assignment: String,
    repo_name: String,
    semester: String,
    github_token: String
}

impl Extension {
    pub fn get_repo_name(&self) -> &String {
        &self.repo_name
    }

    pub fn get_github_token(&self) -> &String {
        &self.github_token
    }

    pub fn format_date(&self) -> String {
        format!("{:04}-{:02}-{:02}T23:59:59", self.year, self.month, self.day)
    }

    pub fn get_netids(&self) -> &Vec<String> {
        &self.netids
    }

    pub fn get_info_assessment_path(&self) -> String {
        format!("courseInstances/{}/assessments/{}/infoAssessment.json", self.semester, self.assignment)
    }
}
