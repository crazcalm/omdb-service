use crate::cache::AppCache;
use kv_log_macro::{info, warn};
use serde_derive::{Deserialize, Serialize};
use std::convert::Infallible;
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IMDBScore {
    name: String,
    imdb_score: String,
}

pub async fn media_info(name: String, app_cache: AppCache) -> Result<impl warp::Reply, Infallible> {
    let mut local_cache = app_cache.lock().await;

    match local_cache.check(name.clone()) {
        Some(value) => {
            info!("Found '{}' in the cache!", &name);

            let result: IMDBScore = serde_json::from_value(value).unwrap();

            Ok(warp::reply::json(&result))
        }
        None => {
            info!("Adding '{}' to the cache", &name);

            let result = omdb_call(name.clone()).await;
            let remove_from_cache = local_cache.add(name.clone(), result.clone());

            if let Some(cache_item) = remove_from_cache {
                info!("Dropping {} from the cache", cache_item.key);
            };

            Ok(warp::reply::json(&result))
        }
    }
}

async fn omdb_call(name: String) -> IMDBScore {
    let api_key = env::var("OMDB_KEY").expect("'OMDB_KEY' was not loaded into the environment");

    let show = omdb::title(&name).apikey(&api_key).get().await;

    match show {
        Ok(item) => IMDBScore {
            name: name.clone(),
            imdb_score: item.imdb_rating,
        },
        Err(err) => {
            warn!("omdb error when searching for '{}': {}", &name, err);

            IMDBScore {
                name: name.clone(),
                imdb_score: "Not Found".to_string(),
            }
        }
    }
}
