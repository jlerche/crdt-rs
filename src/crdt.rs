

pub trait CRDT<T: OpMutation, U> {
    fn new() -> Self;

    fn mutate(&mut self, op: T) -> Self;

    fn query(&self) -> U;

    fn equals(&self, other: Self) -> bool;
}

pub trait OpMutation {}

pub trait StateCRDT<T: OpMutation> {
    fn delta_mutate(&self) -> Self;

    fn merge(&mut self, other: Self) -> Self;

    fn is_bottom(&self) -> bool;

    fn is_inflation(&self, other: Self) -> bool;

    fn is_strict_inflation(&self, other: Self) -> bool;

    fn irreducible_is_strict_inflation();

    fn digest(&self);

    fn join_decomposition();

    fn delta();


}