use crate::cache::AppCache;
use crate::routes;
use warp::Filter;

pub fn register(
    app_cache: AppCache,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let health_endpoints = routes::health::register();
    let omdb_endpoints = routes::omdb::register(app_cache);

    health_endpoints.or(omdb_endpoints)
}
