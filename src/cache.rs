use chrono::prelude::*;
use std::collections::HashMap;

use std::sync::Arc;
use tokio::sync::Mutex;

pub type AppCache = Arc<Mutex<Cache>>;

pub struct CacheReturn {
    pub key: String,
    pub value: CacheItem,
}

#[derive(Debug, Clone)]
pub struct CacheItem {
    result: serde_json::Value,
    date: DateTime<Utc>,
}

pub struct Cache {
    items: HashMap<String, CacheItem>,
    size: usize,
}

impl Cache {
    pub fn new(size: usize) -> Cache {
        Cache {
            size,
            items: HashMap::new(),
        }
    }

    pub fn new_app_cache(size: usize) -> AppCache {
        Arc::new(Mutex::new(Cache::new(size)))
    }

    pub fn check(&self, key: String) -> Option<serde_json::Value> {
        let result = self.items.get(key.as_str());

        match result {
            Some(x) => Some(x.result.clone()),
            None => None,
        }
    }

    pub fn add<T: serde::Serialize>(&mut self, key: String, result: T) -> Option<CacheReturn> {
        let mut return_value: Option<CacheReturn> = None;

        self.items.insert(
            key,
            CacheItem {
                date: Utc::now(),
                result: serde_json::json!(result),
            },
        );

        // Still need to enforce size limit
        if self.items.len() > self.size {
            let mut result_key: Option<String> = None;
            let mut result_value: Option<&CacheItem> = None;

            for (key, value) in self.items.iter() {
                if result_value.clone().is_none() {
                    result_key = Some(key.to_string());
                    result_value = Some(value);
                } else {
                    if result_value.unwrap().date > value.date {
                        result_key = Some(key.to_string());
                        result_value = Some(value);
                    }
                }
            }

            // removing the excess item
            if result_key.is_some() {
                let key = result_key.unwrap();
                let value = self.items.remove(&key).unwrap();

                return_value = Some(CacheReturn { key, value });
            }
        }

        return_value
    }
}

#[cfg(test)]
mod tests {
    use crate::cache;

    #[tokio::test]
    async fn test_cache_size() {
        let mut cache = cache::Cache::new(3);

        let one = cache.add("one".to_string(), "1".to_string());
        assert_eq!(one.is_none(), true);

        let two = cache.add("two".to_string(), "2".to_string());
        assert_eq!(two.is_none(), true);

        let three = cache.add("three".to_string(), "3".to_string());
        assert_eq!(three.is_none(), true);

        let four = cache.add("four".to_string(), "4".to_string());
        assert_eq!(four.is_some(), true);
        assert_eq!(four.unwrap().value.result, "1".to_string());

        let five = cache.add("five".to_string(), "5".to_string());
        assert_eq!(five.is_some(), true);
        assert_eq!(
            five.unwrap().value.result,
            serde_json::json!("2".to_string())
        );
    }
}
