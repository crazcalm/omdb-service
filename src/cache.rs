use chrono::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct CacheItem {
    result: String,
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

    pub fn len(self) -> usize {
        self.size
    }

    pub fn check(&self, key: String) -> Option<String> {
        let result = self.items.get(key.as_str());

        match result {
            Some(x) => Some(x.result.clone()),
            None => None,
        }
    }

    pub fn add(&mut self, key: String, result: String) -> Option<String> {
        let mut return_value: Option<String> = None;

        self.items.insert(
            key,
            CacheItem {
                date: Utc::now(),
                result,
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
                let cached_item = self.items.remove(result_key.unwrap().as_str());

                return_value = Some(cached_item.unwrap().result.clone());
            }
        }

        return_value
    }
}

#[cfg(test)]
mod tests {
    use crate::cache;

    fn test_cache_size() {
        let mut cache = cache::Cache::new(3);

        let one = cache.add("one".to_string(), "1".to_string());
        assert_eq!(one, None);

        let two = cache.add("two".to_string(), "2".to_string());
        assert_eq!(two, None);

        let three = cache.add("three".to_string(), "3".to_string());
        assert_eq!(three, None);

        let four = cache.add("four".to_string(), "4".to_string());
        assert_eq!(four, Some("1".to_string()));

        let five = cache.add("five".to_string(), "5".to_string());
        assert_eq!(five, Some("five".to_string()));
    }
}
