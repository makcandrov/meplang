use core::hash::Hash;
use std::collections::{HashMap, HashSet};

#[derive(Default, Clone, Debug)]
pub struct DedupQueue<T> {
    queue: Vec<T>,
    indexes: HashMap<T, usize>,
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
            self.indexes.insert(item.clone(), self.queue.len());
            self.queue.push(item);
            true
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.queue.pop().map(|item| {
            self.indexes.remove(&item);
            item
        })
    }

    // pub fn mark_as_seen(&mut self, item: T) {
    //     self.seen.insert(item.clone());
    //     if let Some(index) = self.indexes.remove(&item) {
    //         let queue_len = self.queue.len();
    //         self.queue.swap(index, queue_len);
    //         self.queue.pop();
    //     }
    // }

    pub fn seen(&self, item: &T) -> bool {
        self.seen.contains(item)
    }
}
