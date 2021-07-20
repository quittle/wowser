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
    K: Hash + Eq + Clone,
    Compute: Fn(&K) -> Option<V>,
{
    map: HashMap<K, Rc<V>>,
    accessed: VecDeque<K>,
    capacity: usize,
    compute: Compute,
}

impl<K, V, Compute> LRUCache<K, V, Compute>
where
    K: Hash + Eq + Clone,
    Compute: Fn(&K) -> Option<V>,
{
    /// Creates a new LRU cache, that auto-fills using `compute` when when not present
    pub fn new(capacity: NonZeroUsize, compute: Compute) -> LRUCache<K, V, Compute>
    where
        Compute: Fn(&K) -> Option<V>,
    {
        LRUCache {
            map: HashMap::new(),
            accessed: VecDeque::new(),
            capacity: capacity.get(),
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
    pub fn get(&mut self, key: &K) -> Option<Weak<V>> {
        let cached_value = self.map.get(key);

        if let Some(v) = cached_value {
            // Clone the Rc to allow record_acces to run on self since the value is no longer
            // borrowed
            let v = v.clone();
            self.record_access(key);
            return Some(Rc::downgrade(&v));
        }

        let value = Rc::new((self.compute)(key)?);
        self.record_access(key);
        let weak_value = Rc::downgrade(&value);
        self.map.insert(key.clone(), value);

        // Drop old value if filled beyond capacity
        if self.accessed.len() > self.capacity {
            if let Some(key_to_drop) = self.accessed.pop_back() {
                self.map.remove(&key_to_drop);
            }
        }

        Some(weak_value)
    }

    fn record_access(&mut self, key: &K) {
        let accessed = &mut self.accessed;

        debug_assert_eq!(accessed.len(), self.map.len());
        debug_assert!(accessed.len() <= self.capacity);

        // Update keeping track of what was least recently used
        if let Some(index) = accessed.get_index(key) {
            accessed.remove(index);
        }
        accessed.push_front(key.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut cache = LRUCache::new(NonZeroUsize::new(1).unwrap(), |v| Some(-v));
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
        assert_eq!(-100, *cache.get(&100).unwrap().upgrade().unwrap());
        assert_eq!(cache.len(), 1);
        assert!(!cache.is_empty());
    }

    #[test]
    fn test_failed_to_compute() {
        static mut COUNT: i32 = 0;
        let mut cache = LRUCache::new(NonZeroUsize::new(5).unwrap(), |v| unsafe {
            COUNT += 1;
            if COUNT % 2 == 0 {
                Some(*v)
            } else {
                None
            }
        });
        assert_eq!(unsafe { COUNT }, 0);

        // First compute fails
        assert!(cache.get(&1).is_none());
        assert_eq!(unsafe { COUNT }, 1);

        // Second compute should succeed
        assert!(cache.get(&1).is_some());
        assert_eq!(unsafe { COUNT }, 2);

        // Third get, cache hit, should succeed
        assert!(cache.get(&1).is_some());
        assert_eq!(unsafe { COUNT }, 2);

        // Fourth get, cache, miss should fail
        assert!(cache.get(&2).is_none());
        assert_eq!(unsafe { COUNT }, 3);

        // Fifth get, cache hit
        assert!(cache.get(&2).is_some());
        assert_eq!(unsafe { COUNT }, 4);
    }

    #[test]
    fn test_fill_and_bust_cache() {
        static mut COUNT: i32 = 0;
        let mut cache = LRUCache::new(NonZeroUsize::new(2).unwrap(), |v| {
            unsafe { COUNT += 1 };
            Some(-v)
        });
        assert_eq!(cache.len(), 0);
        assert_eq!(0, unsafe { COUNT });

        // Normal filling of the cache
        cache.get(&0);
        assert_eq!(cache.len(), 1);
        assert_eq!(1, unsafe { COUNT });
        cache.get(&1);
        assert_eq!(cache.len(), 2);
        assert_eq!(2, unsafe { COUNT });

        // Should be cache hits with the previously computed value
        cache.get(&1);
        cache.get(&1);
        assert_eq!(cache.len(), 2);
        assert_eq!(2, unsafe { COUNT });

        // Compute a new value and drop 0
        cache.get(&2);
        assert_eq!(cache.len(), 2);
        assert_eq!(3, unsafe { COUNT });

        // No need to recompute 1
        cache.get(&1);
        assert_eq!(cache.len(), 2);
        assert_eq!(3, unsafe { COUNT });

        // 0 should have been dropped and requires recomputing
        cache.get(&0);
        assert_eq!(cache.len(), 2);
        assert_eq!(4, unsafe { COUNT });
    }
}
