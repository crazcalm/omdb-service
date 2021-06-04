use crate::handlers::health_handlers;
use warp::Filter;

pub fn register() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("omdb-service" / "health")
        .and(warp::get())
        .and_then(health_handlers::healthy)
        .with(warp::log("health endpoint"))
}
