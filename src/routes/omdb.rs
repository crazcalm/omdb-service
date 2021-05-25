use crate::handlers::omdb_handlers;
use warp::Filter;

pub fn register() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("omdb-service" / "name" / String)
        .and(warp::get())
        .and_then(omdb_handlers::media_info)
        .with(warp::log("name endpoint"))
}
