use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GenericMap<K, V> {
    keys: Vec<K>,
    vals: Vec<V>,
}

impl<K: PartialEq + PartialOrd, V> GenericMap<K, V> {
    pub fn new() -> GenericMap<K, V> {
        Self {
            keys: Vec::new(),
            vals: Vec::new(),
        }
    }
    pub fn with_capacity(capacity: usize) -> GenericMap<K, V> {
        Self {
            keys: Vec::with_capacity(capacity),
            vals: Vec::with_capacity(capacity),
        }
    }
    pub fn insert(&mut self, key: K, value: V) {
        let index = self.keys.iter().position(|p| p == &key);
        if index.is_some() {
            self.vals[index.unwrap()] = value;
        } else {
            let mut index = 0;
            loop {
                if index == self.keys.len() {
                    break;
                }
                if self.keys[index] < key {
                    break;
                }
                index += 1;
            }
            self.keys.insert(index, key);
            self.vals.insert(index, value);
        }
    }
    pub fn get(&self, key: &K) -> Option<&V> {
        let index = match self.keys.iter().position(|p| p == key) {
            None => return None,
            Some(s) => s,
        };
        Some(&self.vals[index])
    }
}
