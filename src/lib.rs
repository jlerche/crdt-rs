extern crate indexmap;
extern crate itertools;
pub mod dot;
pub mod dot_store;
pub mod causal_context;
pub mod crdt;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
