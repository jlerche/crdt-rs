extern crate indexmap;

use self::indexmap::IndexSet;
use dot::Dot;

pub struct DotSet<T> {
    dots: IndexSet<T>
}

impl DotSet<T> {
    fn new() -> Self {
        DotSet {dots: IndexSet::new() }
    }
}
