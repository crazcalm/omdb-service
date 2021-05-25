use crate::handlers::health_handlers;
use crate::routes::omdb;
use warp::Filter;

pub fn register() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let health = warp::path!("omdb-service" / "health")
        .and(warp::get())
        .and_then(health_handlers::healthy)
        .with(warp::log("health endpoint"));

    let omdb_endpoints = omdb::register();

    health.or(omdb_endpoints)
}
