

pub trait CRDT<T: OpMutation, U> {
    fn new() -> Self;

    fn mutate(&mut self, op: T) -> Self;

    fn query(&self) -> U;

    fn equals(&self, other: Self) -> bool;
}

pub trait OpMutation {}