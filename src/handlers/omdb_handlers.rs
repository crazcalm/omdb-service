use crate::cache::AppCache;
use kv_log_macro::info;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::convert::Infallible;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IMDBScore {
    name: String,
    score: String,
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

            let result = dummy_function(name.clone()).await;
            let remove_from_cache = local_cache.add(name.clone(), result.clone());

            if remove_from_cache.is_some() {
                let cache_item = remove_from_cache.unwrap();

                info!("Dropping {} from the cache", cache_item.key);
            }

            Ok(warp::reply::json(&result))
        }
    }

    /*
    let dummy_data = dummy_function(name).await;

    let warp_reply = warp::reply::json(&dummy_data);

    Ok(warp_reply)

    */
}

async fn dummy_function(name: String) -> IMDBScore {
    IMDBScore {
        name: "name".clone().to_string(),
        score: format!("{} + score", name),
    }
}
