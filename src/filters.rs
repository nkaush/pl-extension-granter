use super::models::Extension;
use super::handlers;
use warp::Filter;

/// All filters combined
pub fn all() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    grant_extensions()
        // .or(function)
}

/// POST /api/grant with JSON body
pub fn grant_extensions() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
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
