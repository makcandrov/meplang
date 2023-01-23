use std::fmt::Debug;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug, Clone)]
pub struct DependencyTree<T: Debug + Clone> {
    parents: HashMap<T, HashSet<T>>,
    children: HashMap<T, HashSet<T>>,
    leaves: HashSet<T>,
}

impl<T: Debug + Clone> Default for DependencyTree<T> {
    fn default() -> Self {
        Self { parents: HashMap::new(), children: HashMap::new(), leaves: HashSet::new() }
    }
}

impl<T: Debug + Clone + Eq + Hash> DependencyTree<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node_if_needed(&mut self, item: &T) {
        if let Some(set) = self.children.get(item) {
            if set.len() > 0 {
                return;
            }
        }
        self.leaves.insert(item.clone());
    }

    pub fn insert_if_needed(&mut self, item: &T, dependency: &T) -> bool {
        let res = self.insert_child(item, dependency);
        assert!(self.insert_parent(item, dependency));
        if let Some(set) = self.children.get(dependency) {
            if set.len() > 0 {
                self.leaves.remove(dependency);
                return res;
            }
        }
        self.leaves.insert(dependency.clone());
        res
    }

    pub fn pop_leaf(&mut self) -> Option<T> {
        let Some(leaf) = self.leaves.iter().next().cloned() else {
            return None;
        };
        self.leaves.remove(&leaf);
        assert!(self.children.get(&leaf).is_none());
        if let Some(parents) = self.parents.remove(&leaf) {
            for parent in parents {
                let set = self.children.get_mut(&parent).unwrap();
                if set.len() == 1 {
                    assert!(self.children.remove(&parent).is_some());
                    self.leaves.insert(parent);
                } else {
                    assert!(set.remove(&leaf));
                }
            }
        }
        Some(leaf)
    }

    pub fn is_empty(&self) -> bool {
        self.children.len() == 0 && self.parents.len() == 0
    }

    pub fn leaves(&self) -> &HashSet<T> {
        &self.leaves
    }

    fn insert_child(&mut self, parent: &T, child: &T) -> bool {
        insert_or_create(&mut self.children, parent, child)
    }

    fn insert_parent(&mut self, parent: &T, child: &T) -> bool {
        insert_or_create(&mut self.parents, child, parent)
    }
}

fn insert_or_create<X, Y>(hash_map: &mut HashMap<X, HashSet<Y>>, x: &X, y: &Y) -> bool
where
    X: Clone + Eq + Hash,
    Y: Clone + Eq + Hash,
{
    let Some(set) = hash_map.get_mut(x) else {
        hash_map.insert(x.clone(), [y.clone()].into());
        return true;
    };
    if set.contains(y) {
        return false;
    }
    set.insert(y.clone());
    return true;
}
