use std::cmp::Eq;
use std::hash::Hash;

use indexmap::IndexSet;
use dot::Dot;

pub trait DotStore {
    fn new() -> Self;

    fn is_empty(&self) -> bool;
}

pub struct DotSet<T: Hash + Eq + Clone> {
    dots: IndexSet<Dot<T>>
}

impl<T: Hash + Eq + Clone> DotStore for DotSet<T> {
    fn new() -> DotSet<T> {
        DotSet { dots: IndexSet::new() }
    }

    fn is_empty(&self) -> bool {
        self.dots.is_empty()
    }
}


impl<T: Hash + Eq + Clone> DotSet<T> {
    pub fn from_dots<I: IntoIterator<Item=Dot<T>>>(dots: I) -> DotSet<T> {
        let mut dotset = Self::new();
        for dot in dots.into_iter() {
            dotset.add_dot(dot);
        }
        dotset
    }

    pub fn add_dot(&mut self, dot: Dot<T>) {
        self.dots.insert(dot);
    }


    pub fn has_element(&self, dot: &Dot<T>) -> bool {
        self.dots.contains(dot)
    }

    pub fn union(&mut self, other: DotSet<T>) {
        self.dots = self.dots.union(&other.dots).cloned().collect();
    }

    pub fn intersection(&mut self, other: DotSet<T>) {
        self.dots = self.dots.intersection(&other.dots).cloned().collect();
    }

    pub fn subtract(&mut self, other: DotSet<T>) {
        self.dots = self.dots.difference(&other.dots).cloned().collect();
    }

    // fn subtract_causal_context <- after implementing causal_context

    pub fn fold<F, U>(&self, f: F, acc: U) -> U
        where F: FnMut(U, &Dot<T>) -> U
    {
        self.dots.iter().fold(acc, f)
    }
}
