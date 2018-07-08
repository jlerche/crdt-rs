use std::cmp::Eq;
use std::hash::Hash;
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Dot<T>
where
    T: Eq + Hash + Clone,
{
    pub actor: T,
    pub sequence: usize,
}
