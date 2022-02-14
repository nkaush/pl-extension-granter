use std::env;
use std::net::Ipv4Addr;
use warp::Filter;

mod filters;
mod handlers;
mod models;

/// Provides a RESTful web server managing some Todos.
///
/// API will be:
///
/// - `GET /`: serve HTML homepage
/// - `POST /api/grant`: grant extentions to students
#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=extension-granter=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "extension-granter=debug");
    }
    pretty_env_logger::init();

    let api = filters::all();

    // View access logs by setting `RUST_LOG=extension-granter`.
    let routes = api.with(warp::log("extension-granter"));
    // Start up the server...
    let port_key = "PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("PORT is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(routes).run((Ipv4Addr::UNSPECIFIED, port)).await
}
