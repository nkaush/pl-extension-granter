use super::models::{AssessmentInfoFile, Extension, PLCommit};
use serde::Serialize;
use std::convert::Infallible;
use warp::http::StatusCode;

#[derive(Serialize)]
struct ExtensionResponse {
    message: String,
}

impl ExtensionResponse {
    fn new(message: String) -> ExtensionResponse {
        ExtensionResponse { message }
    }
}

pub async fn add_extension(add: Extension) -> Result<impl warp::Reply, Infallible> {
    let repo_owner: String = "PrairieLearn".into();

    let mut assessment_info = match AssessmentInfoFile::get(
        &repo_owner,
        add.get_repo_name(),
        &add.get_info_assessment_path(),
        add.get_github_token(),
    ).await {
        Ok(ai) => ai,
        Err(_) => {
            let message: String = format!(
                "Could not access file {} in GitHub repository {}!",
                add.get_info_assessment_path(),
                add.get_repo_name()
            );
            return Ok(warp::reply::with_status(
                warp::reply::json(&ExtensionResponse::new(message)),
                StatusCode::FORBIDDEN,
            ));
        }
    };

    assessment_info.grant_extensions(&add);

    let commit: PLCommit = PLCommit::new(
        add.get_repo_name().clone(),
        repo_owner,
        add.get_info_assessment_path(),
        assessment_info.get_sha().clone(),
        assessment_info.get_content().clone(),
        format!(
            "Granting extensions for {} to {} at {}",
            add.get_assignment(),
            add.format_netids(),
            chrono::Utc::now()
        ),
        "Neil Kaushikkar".into(),
        "neil.kaushikkar@gmail.com".into(),
    );

    match commit.make(add.get_github_token().clone()).await {
        Ok(_) => {
            let message: String = format!(
                "Successfully granted extension on {} to {} at {}",
                add.get_assignment(),
                add.format_netids(),
                chrono::Utc::now()
            );
            return Ok(warp::reply::with_status(
                warp::reply::json(&ExtensionResponse::new(message)),
                StatusCode::CREATED,
            ));
        }
        Err(_) => {
            let message: String = format!(
                "Could not access file {} in GitHub repository {}!",
                add.get_info_assessment_path(),
                add.get_repo_name()
            );
            return Ok(warp::reply::with_status(
                warp::reply::json(&ExtensionResponse::new(message)),
                StatusCode::FORBIDDEN,
            ));
        }
    }
}
