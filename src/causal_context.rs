use std::cmp::Eq;
use std::hash::Hash;
use std::mem;

use indexmap::{IndexMap, IndexSet};

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

    pub fn from_dot_set(dot_set: DotSet<T>) -> CausalContext<T> {
        let mut cc = Self::new();
        for dot in dot_set.dots.into_iter() {
            cc.add_dot(dot);
        }
        cc
    }

    pub fn dots(&self) -> DotSet<T> {
        let mut acc = DotSet::new();
        for (actor, sequence) in &self.compressed {
            for num in 1..=*sequence {
                acc.add_dot(Dot {actor: actor.clone(), sequence: num});
            }
        }
        acc.union(&self.dot_set);
        acc
    }

    pub fn add_dot(&mut self, dot: Dot<T>) {
        let current = self.compressed.get(&dot.actor).cloned().unwrap_or(0);

        if dot.sequence == current + 1 {
            let next_dot = Dot {actor: dot.actor.clone(), sequence: dot.sequence + 1};
            self.compressed.insert(dot.actor, dot.sequence);
            if self.dot_set.has_element(&next_dot) {
                self.compress();
            }
        } else {
            if dot.sequence > current + 1 {
                self.dot_set.add_dot(dot);
            }
        }
    }

    pub fn next_dot(&self, actor: T) -> Dot<T> {
        let max_value = self.compressed.get(&actor).unwrap_or(&0);
        Dot { actor, sequence: max_value + 1}
    }

    pub fn is_empty(&self) -> bool {
        self.compressed.is_empty() && self.dot_set.is_empty()
    }

    pub fn has_element(&self, dot: Dot<T>) -> bool {
        let current = *self.compressed.get(&dot.actor).unwrap_or(&0);
        dot.sequence <= current || self.dot_set.has_element(&dot)
    }

    pub fn union(&mut self, other: &Self) -> Self {
        self.compressed.extend(other.compressed).
    }

    fn compress(&mut self) {
        let old = mem::replace(&mut self.dot_set.dots, IndexSet::new());
        for dot in old {
            self.add_dot(dot);
        }
    }
}
