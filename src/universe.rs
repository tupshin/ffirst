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

#[derive(Clone)]
pub struct BTreeBackedSack<C, D, T> {
    t: T,
    b: BTreeMap<C, D>,
}

impl<C,T> SackLike<C,(),C> for BTreeBackedSack<C,(),T> where C:SackBacker,T:TokenLike{}



// #[derive(Clone)]
// pub struct Sack<C, D, B, T>
//    where B: SackBacker
// {
//    b: B,
//    t: T,
//    _phantom: PhantomData<(C, D)>,
// }

impl<C, D, T> SackLike<C, D, BTreeBackedSack<C, D,T>> for BTreeBackedSack<C, D, T>
    where C: SackStorable,
          D: SackStorable,
          T: TokenLike + SackStorable
{
}


impl<S1, S2, S3, C1, C2, C3, D1, D2, D3, B1, B2, B3, T1, T2, T3> Decayer<S1, S2, S3, C1, C2, C3, D1, D2, D3, B1, B2, B3, T1, T2, T3>
    for (S1,S2,S3)
    where S1: SackLike<C1, D1, B1>,
          S2: SackLike<C2, D2, B2>,
          S3: SackLike<C3, D3, B3>,
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
    fn decay<'a>(_s1: &'a S1) -> (&'a S2, &'a S3) {
        unimplemented!()
    }
}

impl<S1, S2, C1, C2, D1, D2, B1, B2, T1, T2> Emitter<S1, S2, C1, C2, D1, D2, B1, B2, T1, T2> for S1
    where T1: Default,
          S1: SackLike<C1, D1, B1>,
          S2: SackLike<C2, D2, B2>,
          S1: Decayer<S1, S1, S2, C1, C1, C2, D1, D1, D2, B1, B1, B2, T1, T1, T2>,
          C1: SackStorable,
          C2: SackStorable,
          D1: SackStorable,
          D2: SackStorable,
          B1: SackBacker,
          B2: SackBacker,
          T1: TokenLike,
          T2: TokenLike
{
    fn emit<'a>(_s1: &'a S1) -> (&'a S1, &'a S2) {
        unimplemented!()
    }
}

impl<S1,S2,S3,S4,C1, C2, C3, C4, D1, D2, D3, D4, B1, B2, B3, B4, T1, T2, T3, T4>
Collider<S1,S2,S3,S4,C1, C2, C3,C4,D1,D2,D3,D4,B1,B2,B3,B4,T1,T2,T3,T4> for (C1, C2, C3, C4, D1, D2, D3, D4, B1, B2, B3, B4, T1, T2, T3, T4) {
    fn collide<'a>(_s1: &'a SackLike<C1,D1,B1>, _s2: &'a SackLike<C2,D2,B2>) -> (&'a SackLike<C3,D3,B3>, &'a SackLike<C4,D4,B4>) {
        unimplemented!()
    }
}

impl<C, T> SetStore<BTreeBackedSack<C, (), T>,C, T> for BTreeBackedSack<C, (), T>
    where T: TokenLike + Default,
          C: SackBacker + SackStorable + Default+Ord
{
    fn insert(s: BTreeBackedSack<C,(),T>, value: C) -> BTreeBackedSack<C,(),T> {
        let mut b = s.b;
        b.insert(value, ());
        BTreeBackedSack{t:s.t,b:b}
    }
}

impl<C, D, T> Default for BTreeBackedSack<C, D, T>
    where T: Default,
          C: Default+Ord
{
    fn default() -> BTreeBackedSack<C,D,T> {
        BTreeBackedSack {
            t: T::default(),
            b: BTreeMap::new(),
        }
    }
}
