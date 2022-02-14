use super::handlers;
use super::models::Extension;
use warp::Filter;

/// All filters combined
pub fn all() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    home()
        .or(serve_static())
        .or(grant_extensions())
        .or(favicon())
}

/// GET /
pub fn home() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end().and(warp::fs::file("templates/index.html"))
}

/// GET /static/
pub fn serve_static() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("static").and(warp::fs::dir("www/static"))
}

/// GET /favicon.ico
pub fn favicon() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("favicon.ico").and(warp::fs::file("www/static/favicon.png"))
}

/// POST /api/grant with JSON body
pub fn grant_extensions() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path!("api" / "grant")
        .and(warp::post())
        .and(json_body())
        .and_then(handlers::add_extension)
}

fn json_body() -> impl Filter<Extract = (Extension,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
