use std::cmp::Eq;
use std::hash::Hash;

use indexmap::IndexMap;

use dotsset::{DotSet, DotStore};
use dot::Dot;

pub struct CausalContext<T: Eq + Hash + Clone> {
    compressed: IndexMap<T, usize>,
    dot_set: DotSet<T>,
}

impl<T: Hash + Eq + Clone> CausalContext<T> {
    pub fn new() -> CausalContext<T> {
        CausalContext {
            compressed: IndexMap::new(),
            dot_set: DotSet::new(),
        }
    }

//    pub fn from_dot_set(dot_set: DotSet<T>) -> CausalContext<T> {
//
//    }

//    pub fn add_dot(&mut self, dot: Dot<T>) {
//        let current = self.compressed.get(&dot.actor).or(Some(&0 as &usize));
//
//        if dot.sequence == current + 1 {
//            self.compressed.insert(dot.actor, dot.sequence);
//
//
//        }
//    }
//
//    fn compress
}
