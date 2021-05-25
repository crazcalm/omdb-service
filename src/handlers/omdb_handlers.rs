use std::collections::HashMap;
use std::convert::Infallible;

pub async fn media_info(name: String) -> Result<impl warp::Reply, Infallible> {
    let dummy_data = dummy_function(name).await;

    Ok(warp::reply::json(&dummy_data))
}

async fn dummy_function(name: String) -> HashMap<String, String> {
    let mut data = HashMap::new();
    data.insert("name".to_string(), name);

    data
}
