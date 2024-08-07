use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    let mut map = OurMap::new();
    map.insert(1, 2);
    let value = map.get(1);
    assert!(value == Some(2));
    let value2 = map.get(5);
    assert!(value2.is_none());
}

struct OurMap<K, V> {
    buckets: Vec<Option<(K, V)>>,
}

const INITIAL_BUCKET_SIZE: usize = 5381;

impl<K: Hash, V: Clone> OurMap<K, V> {
    fn new() -> OurMap<K, V> {
        let mut buckets = Vec::with_capacity(INITIAL_BUCKET_SIZE);
        for _ in 0..INITIAL_BUCKET_SIZE {
            buckets.push(None);
        }
        OurMap { buckets }
    }

    fn insert(&mut self, key: K, value: V) {
        let index = self.get_index_for_key(&key);
        self.buckets[index] = Some((key, value))
    }

    fn get_index_for_key(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % self.buckets.len() as u64) as usize
    }

    fn get(&self, key: K) -> Option<&V> {
        let index = self.get_index_for_key(&key);
        match self.buckets.get(index) {
            Some(Some((_, v))) => Some(v),
            _ => None,
        }

        // second iteration
        // self.buckets
        //     .iter()
        //     .find(|pair| pair.0 == key)
        //     .map(|pair| pair.1)

        // first iteration
        // for pair in &self.buckets {
        //     if pair.0 == key {
        //         return Some(pair.1);
        //     }
        // }
        //
        // None
    }
}
