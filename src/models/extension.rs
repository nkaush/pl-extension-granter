use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Extension {
    netids: String,
    date: String,
    time: String,
    assignment: String,
    repo_name: String,
    semester: String,
    github_token: String,
    credit: u64
}

impl Extension {
    pub fn new(
        netids: String,
        date: String,
        time: String,
        assignment: String,
        repo_name: String,
        semester: String,
        github_token: String,
        credit: u64
    ) -> Self {
        Self {
            netids,
            date,
            time,
            assignment,
            repo_name,
            semester,
            github_token,
            credit
        }
    }

    pub fn get_repo_name(&self) -> &String {
        &self.repo_name
    }

    pub fn get_github_token(&self) -> &String {
        &self.github_token
    }

    pub fn get_assignment(&self) -> &String {
        &self.assignment
    }

    pub fn get_credit(&self) -> u64 {
        self.credit
    }

    pub fn format_date(&self) -> String {
        format!("{}T{}", self.date, self.time)
    }

    pub fn get_netids(&self) -> Vec<String> {
        self.netids
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    }

    pub fn format_netids(&self) -> String {
        self.netids
            .split(',')
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn get_info_assessment_path(&self) -> String {
        format!(
            "courseInstances/{}/assessments/{}/infoAssessment.json",
            self.semester, self.assignment
        )
    }
}
