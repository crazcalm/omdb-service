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

        result.map(|x| x.result.clone())
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
            let key_to_remove = self
                .items
                .iter()
                .reduce(|a, b| if a.1.date > b.1.date { b } else { a }) // compare CacheItem.date
                .map(|result| result.0.clone()) // returns a clone of the key
                .unwrap();

            let value = self.items.remove(key_to_remove.as_str()).unwrap();

            return_value = Some(CacheReturn {
                key: key_to_remove,
                value,
            })
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
