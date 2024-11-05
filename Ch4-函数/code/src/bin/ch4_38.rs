use std::iter::{Filter, Map};

pub trait Iterator {
    type Item;

    fn for_each<F>(self, f: F)
    where
        Self: Sized,
        F: FnMut(Self::Item);

    fn map<B, F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B;
    fn filter<P>(self, predicate: P) -> Filter<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool;
    // other methods...
}

fn main() {}
