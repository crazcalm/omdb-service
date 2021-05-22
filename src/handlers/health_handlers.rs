use crate::models::health_models::Health;
use std::convert::Infallible;

pub async fn healthy() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&Health {
        health: "good".to_string(),
    }))
}
