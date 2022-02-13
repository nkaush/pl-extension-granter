use octocrab::models::repos::{GitUser, FileUpdate};
use octocrab::Octocrab;

#[derive(Debug)]
pub struct PLCommit {
    repo: String,
    owner: String,
    path: String,
    sha: String, 
    new_content: String,
    message: String, 
    name: String,
    email: String
}

impl PLCommit {
    pub fn new(
        repo: String,
        owner: String,
        path: String,
        sha: String, 
        new_content: String,
        message: String, 
        name: String,
        email: String
    ) -> Self { 
        PLCommit {
            repo,
            owner,
            path,
            sha,
            new_content,
            message,
            name,
            email
        }
    }

    pub async fn make(&self) -> Result<FileUpdate, octocrab::Error> {
        let token = std::env::var("GITHUB_TOKEN")
            .expect("GITHUB_TOKEN env variable is required");

        Ok(Octocrab::builder()
            .personal_token(token).build().unwrap()
            .repos(&self.owner, &self.repo)
            .update_file(
                &self.path,
                &self.message,
                &self.new_content,
                &self.sha
            )
            .branch("master")
            .commiter(GitUser {
                name: self.name.clone(),
                email: self.email.clone(),
            })
            .author(GitUser {
                name: self.name.clone(),
                email: self.email.clone(),
            })
            .send()
            .await?
        )
    }
}
