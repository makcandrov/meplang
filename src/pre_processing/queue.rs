use std::collections::HashSet;
use core::hash::Hash;

#[derive(Default, Clone, Debug)]
pub struct DedupQueue<T> {
    queue: Vec<T>,
    seen: HashSet<T>,
}

impl<T: Default> DedupQueue<T>  {
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
            self.queue.push(item);
            true
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.queue.pop()
    }

    pub fn seen(&self, item: &T) -> bool {
        self.seen.contains(item)
    }
}
