use serde::{Serialize, Deserialize};
use slotmap::{DenseSlotMap, SecondaryMap, new_key_type};

new_key_type! {
    struct Key;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SlottedMap<K,V> {
    kmap: DenseSlotMap<Key,K>,
    vmap: SecondaryMap<Key,V>
}

impl <K: PartialEq,V> SlottedMap<K, V> {
    pub fn new() -> SlottedMap<K, V> {
        Self {
            kmap: DenseSlotMap::with_key(),
            vmap: SecondaryMap::new(),
        }
    }
    pub fn with_capacity(capacity: usize) -> SlottedMap<K, V> {
        Self {
            kmap: DenseSlotMap::with_capacity_and_key(capacity),
            vmap: SecondaryMap::with_capacity(capacity),
        }
    }
    pub fn insert(&mut self, key: K, value: V) {
        let index = self.kmap.insert(key);
        self.vmap.insert(index, value);
    }
    pub fn get(&self, key: &K) -> Option<&V> {
        //let index = self.kmap.values().find(|x|x==&key);
        for x in self.kmap.iter() {
            if &x.1 == &key {
                return Some(self.vmap.get(x.0).unwrap());
            }
        }
        None
    }
}