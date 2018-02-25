use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;

use std::sync::Arc;

use super::CacheKey;

pub trait Cache<K, T>
where
    K: CacheKey<T> + Hash + Eq + Clone,
{
    fn get(&self, key: &K) -> Option<Arc<T>>;

    fn insert(&mut self, key: K, value: T) -> Option<Arc<T>>;

    fn remove(&mut self, key: &K) -> Option<Arc<T>>;

    fn clear(&mut self);
}

pub struct HashCache<K, T>(HashMap<K, Arc<T>>)
where
    K: CacheKey<T> + Hash + Eq + Clone;

impl<K, T> Cache<K, T> for HashCache<K, T>
where
    K: CacheKey<T> + Hash + Eq + Clone,
{
    fn get(&self, key: &K) -> Option<Arc<T>> {
        self.0.get(key).map(Arc::clone)
    }

    fn insert(&mut self, key: K, value: T) -> Option<Arc<T>> {
        self.0.insert(key, Arc::new(value))
    }

    fn remove(&mut self, key: &K) -> Option<Arc<T>> {
        self.0.remove(key)
    }

    fn clear(&mut self) {
        self.0.clear()
    }
}

impl<K, T> Default for HashCache<K, T>
where
    K: CacheKey<T> + Hash + Eq + Clone,
{
    fn default() -> Self {
        HashCache(Default::default())
    }
}
