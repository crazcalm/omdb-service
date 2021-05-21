use warp::Filter;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let api = health::register();

    warp::serve(api).run(([127, 0, 0, 1], 3030)).await;
}

mod health {
    use super::health_handlers;
    use warp::Filter;

    pub fn register() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("health")
            .and(warp::get())
            .and_then(health_handlers::healthy)
    }
}

mod health_handlers {
    use super::health_models::Health;
    use std::convert::Infallible;
    use warp::http::StatusCode;

    pub async fn healthy() -> Result<impl warp::Reply, Infallible> {
        Ok(warp::reply::json(&Health {
            health: "good".to_string(),
        }))
    }
}

mod health_models {
    use serde_derive::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Health {
        pub health: String,
    }
}
