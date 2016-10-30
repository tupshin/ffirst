use data_store::set_store::SetStore;
use particle::collide::collide::Collider;
use particle::decay::decay::Decayer;
use particle::decay::emit::Emitter;
use sack::{SackLike, SackBacker};
use sack::SackStorable;

// ----------------------------------------------------------

/// A test concrete implementation

use sack::TokenLike;

use std::collections::BTreeMap;
impl<C, D, T> SackBacker for BTreeBackedSack<C, D, T>
    where C: SackStorable,
          D: SackStorable,
          T: Clone
{
}

#[derive(Clone,Debug)]
pub struct BTreeBackedSack<C, D, T> {
    t: T,
    b: BTreeMap<C, D>,
}

impl<'a, C: 'a, T: 'a> SackLike<'a, C, (), C> for BTreeBackedSack<C, (), T>
    where C: SackBacker,
          T: TokenLike
{
}



// #[derive(Clone)]
// pub struct Sack<C, D, B, T>
//    where B: SackBacker
// {
//    b: B,
//    t: T,
//    _phantom: PhantomData<(C, D)>,
// }

impl<'a, C: 'a, D: 'a, T: 'a> SackLike<'a, C, D, BTreeBackedSack<C, D, T>> for BTreeBackedSack<C, D, T>
    where C: SackStorable,
          D: SackStorable,
          T: TokenLike + SackStorable
{
}


impl<'a, S1:'a, S2:'a, S3:'a, C1:'a, C2:'a, C3:'a, D1:'a, D2:'a, D3:'a, B1:'a, B2:'a, B3:'a, T1:'a, T2:'a, T3:'a> Decayer<'a, S1, S2, S3, C1, C2, C3, D1, D2, D3, B1, B2, B3, T1, T2, T3> for (S1, S2, S3)
    where S1: SackLike<'a, C1, D1, B1>,
          S2: SackLike<'a, C2, D2, B2>,
          S3: SackLike<'a, C3, D3, B3>,
          C1: SackStorable,
          C2: SackStorable,
          C3: SackStorable,
          D1: SackStorable,
          D2: SackStorable,
          D3: SackStorable,
          B1: SackBacker,
          B2: SackBacker,
          B3: SackBacker,
          T1: TokenLike,
          T2: TokenLike,
          T3: TokenLike
{
    fn decay(_s1: &'a S1) -> (&'a S2, &'a S3) {
        unimplemented!()
    }
}

impl<'a, S1: 'a, S2: 'a, C1: 'a, C2: 'a, D1: 'a, D2: 'a, B1: 'a, B2: 'a, T1: 'a, T2: 'a> Emitter<'a, S1, S2, C1, C2, D1, D2, B1, B2, T1, T2> for S1
    where T1: Default,
          S1: SackLike<'a, C1, D1, B1>,
          S2: SackLike<'a, C2, D2, B2>,
          S1: Decayer<'a, S1, S1, S2, C1, C1, C2, D1, D1, D2, B1, B1, B2, T1, T1, T2>,
          C1: SackStorable,
          C2: SackStorable,
          D1: SackStorable,
          D2: SackStorable,
          B1: SackBacker,
          B2: SackBacker,
          T1: TokenLike,
          T2: TokenLike
{
    fn emit(_s1: &'a S1) -> (&'a S1, &'a S2) {
        unimplemented!()
    }
}

impl<'a, C1, C2, C3, C4, D1, D2, D3, D4, B1, B2, B3, B4, T1, T2, T3, T4> Collider<'a, C1, C2, C3, C4, D1, D2, D3, D4, B1, B2, B3, B4, T1, T2, T3, T4>
    for (C1, C2, C3, C4, D1, D2, D3, D4, B1, B2, B3, B4, T1, T2, T3, T4) {
    fn collide(_s1: &'a SackLike<C1, D1, B1>, _s2: &'a SackLike<C2, D2, B2>) -> (&'a SackLike<'a, C3, D3, B3>, &'a SackLike<'a, C4, D4, B4>) {
        unimplemented!()
    }
}

impl<'a, C: 'a, T: 'a> SetStore<'a, BTreeBackedSack<C, (), T>, C, T> for BTreeBackedSack<C, (), T>
    where T: TokenLike + Default,
          C: SackBacker + SackStorable + Default + Ord
{
    fn insert(&'a mut self, value: C) -> &'a Self {
        //  let (mut b,t) = (self.b,self.t);
        self.b.insert(value, ());
        self
    }
}

impl<C, D, T> Default for BTreeBackedSack<C, D, T>
    where T: Default,
          C: Default + Ord
{
    fn default() -> BTreeBackedSack<C, D, T> {
        BTreeBackedSack {
            t: T::default(),
            b: BTreeMap::new(),
        }
    }
}
