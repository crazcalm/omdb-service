mod api;
mod cache;
mod config;
mod handlers;
mod models;
mod routes;

use std::env;

#[tokio::main]
async fn main() {
    // Ensure that a log level is set
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    };

    // intialize json logger
    json_env_logger::init();

    // Ensure that an environment is setup
    if env::var("ENV").is_err() {
        env::set_var("ENV", "local");
    };

    // Get the enviroment variable
    let environment = env::var("ENV").expect("Failed to get the ENV variable");

    config::load_config(environment).await.unwrap();

    // Setup the routes
    let application_api = api::register();

    warp::serve(application_api)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
