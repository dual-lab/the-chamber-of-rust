use std::{collections::HashMap, hash::Hash};

pub struct InMemoryHashMapStorage<K, V> {
    storage: HashMap<K, Vec<V>>,
}

impl<K, V> InMemoryHashMapStorage<K, V>
where
    K: Eq + Hash,
{
    pub fn from(storage: HashMap<K, Vec<V>>) -> Self {
        Self { storage }
    }

    pub fn save(&mut self, key: K, value: V) {
        self.storage.entry(key).or_insert(Vec::new()).push(value);
    }

    pub fn select(&self, key: &K) -> Option<&Vec<V>> {
        self.storage.get(key)
    }
}
