use octocrab::models::repos::{FileUpdate, GitUser};
use octocrab::Octocrab;

#[derive(Debug)]
pub struct PLCommit {
    repo: String,
    owner: String,
    filepath: String,
    sha: String,
    new_content: String,
    message: String,
    name: String,
    email: String,
}

impl PLCommit {
    pub fn new(
        repo: String,
        owner: String,
        filepath: String,
        sha: String,
        new_content: String,
        message: String,
        name: String,
        email: String,
    ) -> Self {
        PLCommit {
            repo,
            owner,
            filepath,
            sha,
            new_content,
            message,
            name,
            email,
        }
    }

    pub async fn make(&self, github_token: String) -> Result<FileUpdate, octocrab::Error> {
        Ok(Octocrab::builder()
            .personal_token(github_token)
            .build()
            .unwrap()
            .repos(&self.owner, &self.repo)
            .update_file(&self.filepath, &self.message, &self.new_content, &self.sha)
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
            .await?)
    }
}
