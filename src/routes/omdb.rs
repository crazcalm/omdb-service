use crate::cache::AppCache;
use crate::handlers::omdb_handlers;
use warp::Filter;

pub fn register(
    app_cache: AppCache,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("omdb-service" / "name" / String)
        .and(warp::get())
        .and(with_cache(app_cache))
        .and_then(omdb_handlers::media_info)
        .with(warp::log("name endpoint"))
}

fn with_cache(
    app_cache: AppCache,
) -> impl Filter<Extract = (AppCache,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || app_cache.clone())
}
