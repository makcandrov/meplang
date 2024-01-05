use std::collections::HashSet;
use std::hash::Hash;

use indexmap::IndexSet;

/// Queue that deduplicates items inserted, even after it is removed.
#[derive(Default, Clone, Debug)]
pub struct PersistentDedupQueue<T> {
    queue: IndexSet<T>,
    seen: HashSet<T>,
}

impl<T: Default> PersistentDedupQueue<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: Eq + Hash + Clone> PersistentDedupQueue<T> {
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
}
