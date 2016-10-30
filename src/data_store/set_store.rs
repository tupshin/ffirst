use sack::{SackLike, SackStorable, TokenLike, SackBacker};
use std::collections::BTreeMap;

pub trait SetStore<'a, S1: 'a, C1: 'a, T1: 'a>: Clone + Default
    where S1: SackLike<'a, C1, (), C1>,
          C1: SackStorable + SackBacker,
          T1: TokenLike
{
    fn insert(&'a mut self, value: C1) -> &'a Self;
}

#[derive(Clone)]
pub struct BTreeBackedSack<C, D, T> {
    t: T,
    b: BTreeMap<C, D>,
}
