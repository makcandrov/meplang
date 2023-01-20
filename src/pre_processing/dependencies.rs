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
        Self {
            parents: HashMap::new(),
            children: HashMap::new(),
            leaves: HashSet::new(),
        }
    }
}

impl<T: Debug + Clone + Eq + Hash> DependencyTree<T> {
    pub fn new() -> Self {
        Self::default()
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
