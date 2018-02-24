use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;

use std::rc::Rc;

use super::CacheKey;

pub trait Cache<K, T>
where
    K: CacheKey<T> + Hash + Eq + Clone,
{
    fn get(&self, key: &K) -> Option<Rc<T>>;

    fn insert(&mut self, key: K, value: T) -> Option<Rc<T>>;

    fn remove(&mut self, key: &K) -> Option<Rc<T>>;

    fn clear(&mut self);
}

pub struct HashCache<K, T>(HashMap<K, Rc<T>>)
where
    K: CacheKey<T> + Hash + Eq + Clone;

impl<K, T> Cache<K, T> for HashCache<K, T>
where
    K: CacheKey<T> + Hash + Eq + Clone,
{
    fn get(&self, key: &K) -> Option<Rc<T>> {
        self.0.get(key).map(Rc::clone)
    }

    fn insert(&mut self, key: K, value: T) -> Option<Rc<T>> {
        self.0.insert(key, Rc::new(value))
    }

    fn remove(&mut self, key: &K) -> Option<Rc<T>> {
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
