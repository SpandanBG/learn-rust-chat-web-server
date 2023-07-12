use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use tokio;

pub struct Cache {
    cached_data: RwLock<HashMap<String, Vec<u8>>>,
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            cached_data: RwLock::new(HashMap::new()),
        }
    }

    pub fn get_data(&self, key: &String) -> Option<Vec<u8>> {
        match self.cached_data.read() {
            Ok(cache_reader) => match cache_reader.get(key) {
                Some(value) => Some(value.clone()),
                _ => None,
            },
            Err(err) => {
                println!("Unable to read from cache => {:.2?}", err);
                None
            }
        }
    }

    async fn async_set_data(cache: Arc<Cache>, key: String, value: Vec<u8>) {
        match cache.cached_data.write() {
            Ok(mut cache_writer) => {
                cache_writer.insert(key, value);
                ()
            }
            Err(err) => {
                println!("Unable to write to cache => {:.2?}", err);
                ()
            }
        }
    }
}

pub struct AsyncCache {
    cache: Arc<Cache>,
}

impl AsyncCache {
    pub fn new() -> Arc<AsyncCache> {
        Arc::new(AsyncCache {
            cache: Arc::new(Cache::new()),
        })
    }

    pub fn set_data(&self, key: &String, value: &Vec<u8>) {
        let key = key.clone();
        let value = value.clone();
        let cache = Arc::clone(&self.cache);
        tokio::spawn(Cache::async_set_data(cache, key, value));
    }

    pub fn get_data(&self, key: &String) -> Option<Vec<u8>> {
        self.cache.get_data(key)
    }
}
