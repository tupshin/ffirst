extern crate sack;


use sack::SackBacker;
use sack::data_store::kv_store::{KVSack,WriteableKVSack};
use std::collections::BTreeMap;

fn main() {
    let s: KVSack<i32, i32, BTreeBackedSack<i32, i32, ()>, ()> = KVSack::default();
  //  let foo:KVSack<i32, i32, BTreeBackedSack<i32, i32, ()>, ()>= WriteableKVSack::insert(Box::new(s),1,2);
}

// impl<C,D> SackBacker for BTreeMap<C,D>{}

#[derive(Clone,Ord,Eq,PartialOrd,PartialEq)]
pub struct BTreeBackedSack<C, D, T> {
    t: T,
    b: BTreeMap<C, D>,
}

impl<C, D, T> SackBacker for BTreeBackedSack<C, D, T>
    where C: Clone,
          D: Clone,
          T: Clone
{
}
impl<C, D, T> Default for BTreeBackedSack<C, D, T>
    where C: Clone+Ord,
          D: Clone,
          T: Clone+Default
{
    fn default() -> Self {
        BTreeBackedSack {
            t: T::default(),
            b: BTreeMap::default(),
        }
    }
}
