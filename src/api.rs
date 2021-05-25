use crate::routes;
use warp::Filter;

pub fn register() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let health_endpoints = routes::health::register();
    let omdb_endpoints = routes::omdb::register();

    health_endpoints.or(omdb_endpoints)
}
