use core::hash::Hash;
use std::collections::{HashMap, HashSet};

#[derive(Default, Clone, Debug)]
pub struct IndexedVec<T> {
    v: Vec<T>,
    indexes: HashMap<T, usize>,
}

impl<T: Default> IndexedVec<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: Eq + Hash + Clone> IndexedVec<T> {
    pub fn insert(&mut self, item: T) -> bool {
        if !self.indexes.contains_key(&item) {
            self.indexes.insert(item.clone(), self.v.len());
            self.v.push(item);
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self, item: &T) -> bool {
        let Some(index) = self.indexes.remove(item) else {
            return false;
        };
        let length = self.v.len();
        self.v.swap(index, length - 1);
        self.v.pop();
        true
    }

    pub fn pop(&mut self) -> Option<T> {
        let Some(item) = self.v.pop() else {
            return None;
        };
        assert!(self.indexes.remove(&item).unwrap() == self.v.len());
        Some(item)
    }

    pub fn as_vec(&self) -> &Vec<T> {
        &self.v
    }
}

#[derive(Default, Clone, Debug)]
pub struct DedupQueue<T> {
    queue: IndexedVec<T>,
    seen: HashSet<T>,
}

impl<T: Default> DedupQueue<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: Eq + Hash + Clone> DedupQueue<T> {
    pub fn insert_if_needed(&mut self, item: T) -> bool {
        if self.seen(&item) {
            false
        } else {
            self.seen.insert(item.clone());
            assert!(self.queue.insert(item));
            true
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.queue.pop()
    }

    pub fn seen(&self, item: &T) -> bool {
        self.seen.contains(item)
    }

    pub fn as_vec(&self) -> &Vec<T> {
        self.queue.as_vec()
    }
}
