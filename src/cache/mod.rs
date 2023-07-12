use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub struct Cache {
    cached_data: RwLock<HashMap<String, Vec<u8>>>,
}

impl Cache {
    pub fn new() -> Arc<Cache> {
        Arc::new(Cache {
            cached_data: RwLock::new(HashMap::new()),
        })
    }

    pub fn set_data(&self, key: String, value: Vec<u8>) {
        match self.cached_data.write() {
            Ok(mut cache_writer) => {
                cache_writer.insert(key, value);
                ()
            }
            Err(err) => {
                println!("Unable to write to cache => {:.2?}", err);
                ()
            }
        };
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
}
