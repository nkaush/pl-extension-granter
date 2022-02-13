use super::models::{Extension, AssessmentInfoFile, PLCommit};
use std::convert::Infallible;
use warp::http::StatusCode;

pub async fn add_extension(add: Extension) -> Result<impl warp::Reply, Infallible> {
    let owner: &str = "PrairieLearn";

    let mut assessment_info = AssessmentInfoFile::get(
            owner, 
            &add.get_repo_name(), 
            &add.get_info_assessment_path())
        .await
        .unwrap();

    assessment_info.grant_extensions(&add);

    let commit: PLCommit = PLCommit::new(
        add.get_repo_name().clone(),
        owner.into(),
        add.get_info_assessment_path(),
        assessment_info.get_sha().clone(), 
        assessment_info.get_content().clone(),
        format!("Granting extensions at {}", chrono::Utc::now()), 
        "Neil Kaushikkar".into(),
        "neil.kaushikkar@gmail.com".into()
    );

    match commit.make().await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Ok(StatusCode::FAILED_DEPENDENCY)
    }
}
