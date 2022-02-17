use futures::executor::block_on;
use handler::models::*;
use std::env;

#[tokio::main]
async fn main() {
    let repo_owner: String = "PrairieLearn".into();
    let add: Extension = Extension::new(
        "neilk3,testing123".into(),
        "2022-02-12".into(),
        "23:59:59".into(),
        "MP0".into(),
        "pl-cs19628".into(),
        "SP22".into(),
        env::var("GITHUB_TOKEN").unwrap(),
    );

    let res = block_on(AssessmentInfoFile::get(
        &repo_owner,
        add.get_repo_name(),
        &add.get_info_assessment_path(),
        add.get_github_token(),
    ));

    let mut assessment_info = match res {
        Ok(ai) => ai,
        Err(_) => {
            let message: String = format!(
                "Could not access file {} in GitHub repository {}!",
                add.get_info_assessment_path(),
                add.get_repo_name()
            );

            println!("{}", message);
            return;
        }
    };

    assessment_info.grant_extensions(&add);

    println!("{}", assessment_info.get_content());
}
