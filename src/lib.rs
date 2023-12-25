mod container;

use std::collections::{HashMap, HashSet};

use container::Container;

// This is simple RoaringBitmap structure for 32bit value
pub struct RoaringBitmap {
    containers: HashMap<u32, Container>,
}

impl RoaringBitmap {
    pub fn new() -> Self {
        Self {
            containers: HashMap::new(),
        }
    }

    pub fn insert(&mut self, i: u32) {
        let index = i >> 16;
        let value = i - index;
        self.containers
            .get_mut(&index)
            .get_or_insert(&mut Container::new())
            .insert(value);
    }

    pub fn remove(&mut self, i: u32) {
        let index = i >> 16;
        let value = i - index;
        self.containers.get_mut(&index).map(|c| c.remove(value));
    }

    pub fn union(&self, other: &RoaringBitmap) -> Self {
        let keys: HashSet<&u32> = self
            .containers
            .keys()
            .chain(other.containers.keys())
            .collect();
        let mut result = RoaringBitmap::new();
        for key in keys {
            let lhs = self.containers.get(key);
            let rhs = other.containers.get(key);
            result.containers.insert(*key, Container::union(lhs, rhs));
        }
        result
    }

    pub fn intersection(&self, other: &RoaringBitmap) -> Self {
        let mut keys = self.containers.keys().collect::<Vec<_>>();
        keys.retain(|k| other.containers.contains_key(*&k));
        let mut result = RoaringBitmap::new();
        for key in keys {
            let lhs = self.containers.get(key);
            let rhs = other.containers.get(key);
            result
                .containers
                .insert(*key, Container::intersection(lhs, rhs));
        }
        result
    }

    pub fn difference(&self, other: &RoaringBitmap) -> Self {
        todo!()
    }

    pub fn symmetric_difference(&self, other: &RoaringBitmap) -> Self {
        todo!()
    }
}
