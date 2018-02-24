use super::*;
use std::hash::Hash;

use std::rc::Rc;

pub trait Loader<K, T>
where
    K: CacheKey<T>,
{
    type Error;

    fn load_resource(&self, key: &K) -> Result<T, Self::Error>;
}

pub trait LoadCache<K, V>: Loader<K, V> + Cache<K, V>
where
    K: CacheKey<V> + Hash + Eq + Clone,
{
    fn get_or_load(&mut self, key: &K) -> Result<Rc<V>, Self::Error> {
        if let Some(value) = self.get(key) {
            return Ok(value);
        }

        let loaded = self.load_resource(key)?;
        self.insert(key.clone(), loaded);
        if let Some(value) = self.get(key) {
            return Ok(value);
        } else {
            unreachable!()
        }
    }
}
