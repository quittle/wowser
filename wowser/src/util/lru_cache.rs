use super::VecDequeExt;
use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
    num::NonZeroUsize,
    rc::{Rc, Weak},
};

/// A least-recently used cache
pub struct LRUCache<K, V, Compute>
where
    K: Hash + Eq + Copy,
    Compute: Fn(&K) -> V,
{
    map: HashMap<K, Rc<V>>,
    accessed: VecDeque<K>,
    capacity: NonZeroUsize,
    compute: Compute,
}

impl<K, V, Compute> LRUCache<K, V, Compute>
where
    K: Hash + Eq + Copy,
    Compute: Fn(&K) -> V,
{
    /// Creates a new LRU cache, that auto-fills using `compute` when when not present
    pub fn new(capacity: NonZeroUsize, compute: Compute) -> LRUCache<K, V, Compute>
    where
        Compute: Fn(&K) -> V,
    {
        LRUCache {
            map: HashMap::new(),
            accessed: VecDeque::new(),
            capacity,
            compute,
        }
    }

    /// The number of items stored in the cache
    pub fn len(&self) -> usize {
        self.accessed.len()
    }

    /// Checks if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.accessed.is_empty()
    }

    /// Gets the item from the cache, computing it when
    pub fn get(&mut self, key: K) -> Weak<V> {
        self.record_access(&key);

        if let Some(v) = self.map.get(&key) {
            return Rc::downgrade(v);
        }
        let value = Rc::new((self.compute)(&key));
        let weak_value = Rc::downgrade(&value);
        self.map.insert(key, value);
        weak_value
    }

    fn record_access(&mut self, key: &K) {
        debug_assert_eq!(self.accessed.len(), self.map.len());
        debug_assert!(self.accessed.len() <= self.capacity.get());

        // Update keeping track of what was least recently used
        let accessed = &mut self.accessed;
        if let Some(index) = accessed.get_index(key) {
            accessed.remove(index);
        }
        accessed.push_front(*key);

        // Drop old value if filled beyond capacity
        if accessed.len() > self.capacity.get() {
            if let Some(key_to_drop) = accessed.pop_back() {
                self.map.remove(&key_to_drop);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut cache = LRUCache::new(NonZeroUsize::new(1).unwrap(), |v| -v);
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
        assert_eq!(-100, *cache.get(100).upgrade().unwrap());
        assert_eq!(cache.len(), 1);
        assert!(!cache.is_empty());
    }

    #[test]
    fn test_fill_and_bust_cache() {
        static mut COUNT: i32 = 0;
        let mut cache = LRUCache::new(NonZeroUsize::new(2).unwrap(), |v| {
            unsafe { COUNT += 1 };
            -v
        });
        assert_eq!(cache.len(), 0);
        assert_eq!(0, unsafe { COUNT });

        // Normal filling of the cache
        cache.get(0);
        assert_eq!(cache.len(), 1);
        assert_eq!(1, unsafe { COUNT });
        cache.get(1);
        assert_eq!(cache.len(), 2);
        assert_eq!(2, unsafe { COUNT });

        // Should be cache hits with the previously computed value
        cache.get(1);
        cache.get(1);
        assert_eq!(cache.len(), 2);
        assert_eq!(2, unsafe { COUNT });

        // Compute a new value and drop 0
        cache.get(2);
        assert_eq!(cache.len(), 2);
        assert_eq!(3, unsafe { COUNT });

        // No need to recompute 1
        cache.get(1);
        assert_eq!(cache.len(), 2);
        assert_eq!(3, unsafe { COUNT });

        // 0 should have been dropped and requires recomputing
        cache.get(0);
        assert_eq!(cache.len(), 2);
        assert_eq!(4, unsafe { COUNT });
    }
}
