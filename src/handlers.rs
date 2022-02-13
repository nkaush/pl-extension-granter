use super::models::{Extension, AssessmentInfoFile, PLCommit};
use std::convert::Infallible;
use serde_json::{Value, Map};
use warp::http::StatusCode;
use handlebars::Handlebars;

pub async fn home() -> Result<impl warp::Reply, Infallible> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("index-template", "templates/index.html").unwrap();

    let parsed: Value = serde_json::from_str("{}").unwrap();
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
    let body: String = handlebars.render("index-template", &obj).unwrap();

    Ok(warp::reply::with_status(warp::reply::html(body), StatusCode::OK))
}

pub async fn add_extension(add: Extension) -> Result<impl warp::Reply, Infallible> {
    println!("{:?}", add);
    let repo_owner: String = "PrairieLearn".into();

    let mut assessment_info = match AssessmentInfoFile::get(
            &repo_owner,
            add.get_repo_name(), 
            &add.get_info_assessment_path(),
            add.get_github_token()
        )
        .await {
            Ok(ai) => ai,
            Err(_) => return Ok(StatusCode::FORBIDDEN)
        };

    assessment_info.grant_extensions(&add);

    let commit: PLCommit = PLCommit::new(
        add.get_repo_name().clone(),
        repo_owner,
        add.get_info_assessment_path(),
        assessment_info.get_sha().clone(), 
        assessment_info.get_content().clone(),
        format!("Granting extensions at {}", chrono::Utc::now()), 
        "Neil Kaushikkar".into(),
        "neil.kaushikkar@gmail.com".into()
    );

    match commit.make(add.get_github_token().clone()).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Ok(StatusCode::FORBIDDEN)
    }
}
