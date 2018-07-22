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

    fn size(&self) -> usize;

    fn drop_unused(&mut self);
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

    fn size(&self) -> usize {
        self.0.len()
    }

    fn drop_unused(&mut self) {
        self.0.retain(|_, value| Arc::strong_count(value) > 1)
    }
}

use std::collections::hash_map::Iter;

impl<'a, K, V> IntoIterator for &'a HashCache<K, V>
where
    K: CacheKey<V> + Hash + Eq + Clone,
{
    type Item = (&'a K, &'a Arc<V>);
    type IntoIter = Iter<'a, K, Arc<V>>;

    fn into_iter(self) -> Iter<'a, K, Arc<V>> {
        self.0.iter()
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

use imgui::Ui;
impl<K, V> HashCache<K, V>
where
    K: CacheKey<V> + Hash + Eq + Clone,
{
    pub fn inspect(
        &mut self,
        ui: &Ui,
        name: &str,
        key_closure: fn(&K) -> &str,
        size_closure: fn(&Arc<V>) -> usize,
    ) {
        use math::format_bytes;

        if ui.collapsing_header(im_str!("{} entries", self.size()))
            .open_on_arrow(true)
            .open_on_double_click(true)
            .build()
        {
            ui.columns(4, im_str!("{}_entries", name), true);
            ui.text(im_str!("Key"));
            ui.next_column();
            ui.text(im_str!("Size"));
            ui.next_column();
            ui.text(im_str!("Strong refs"));
            ui.next_column();
            ui.text(im_str!("Weak refs"));
            ui.separator();

            for (key, value) in self.into_iter() {
                ui.next_column();
                ui.text(im_str!("{}", key_closure(key)));
                ui.next_column();

                let size = size_closure(value);

                ui.text_colored(
                    (0.75, 0.75, 1.0, 1.0),
                    im_str!("{}", format_bytes(size as f64)),
                );
                if ui.is_item_hovered() {
                    ui.tooltip_text(im_str!("{} bytes", size));
                }
                ui.next_column();
                ui.text_colored(
                    (0.0, 1.0, 0.0, 1.0),
                    im_str!("{}", Arc::strong_count(value)),
                );
                ui.next_column();
                ui.text_colored((1.0, 1.0, 0.0, 1.0), im_str!("{}", Arc::weak_count(value)));
            }
            ui.columns(1, im_str!("{}_", name), false);
            ui.new_line();
        }

        if ui.button(im_str!("Empty unused##{}", name), (132.0, 32.0)) {
            self.drop_unused();
        }
    }
}
