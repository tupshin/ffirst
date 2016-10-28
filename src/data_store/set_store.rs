use std::collections::BTreeMap;
use sack::{SackLike, SackStorable, TokenLike, SackBacker};

pub trait SetStore<S1, C1, T1>: Clone+Default
    where S1: SackLike<C1, (), C1>,
          C1: SackStorable + SackBacker,
          T1: TokenLike
{
    fn insert(s1: S1, value: C1) -> S1;
}

#[derive(Clone)]
pub struct BTreeBackedSack<C, D, T> {
    t: T,
    b: BTreeMap<C, D>,
}
