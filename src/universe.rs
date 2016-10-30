use particle::collide::collide::Collider;
use particle::decay::decay::Decayer;
use particle::decay::emit::Emitter;
use sack::{SackType, SackBacker, SackStorable, Sack};

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

// #[derive(Clone,Debug)]
// pub struct BTreeBackedSack<C, D, T> {
//    t: T,
//    b: BTreeMap<C, D>,
// }

// impl<'a, C: 'a, T: 'a> SackLike<'a, C, (), C> for BTreeBackedSack<C, (), T>
//    where C: SackBacker,
//          T: TokenLike
// {
// }

pub type BTreeBackedSack<C, D, T> = Sack<C, D, BTreeMap<C, D>, T>;


// #[derive(Clone)]
// pub struct Sack<C, D, B, T>
//    where B: SackBacker
// {
//    b: B,
//    t: T,
//    _phantom: PhantomData<(C, D)>,
// }


impl<'a,C1:'a, C2:'a, C3:'a, D1:'a, D2:'a, D3:'a, B1:'a, B2:'a, B3:'a, T1:'a, T2:'a, T3:'a>
Decayer<'a, C1, C2, C3, D1, D2, D3, B1, B2, B3, T1, T2, T3> for (SackType<C1, D1, B1>, SackType<C2, D2, B2>,SackType<C3, D3, B3>)
    where          C1: SackStorable,
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
    fn decay(&mut self) -> (&'a SackType<C2, D2, B2>, &'a SackType<C3, D3, B3>) {
        unimplemented!()
    }
}

impl<'a, C1: 'a, C2: 'a, D1: 'a, D2: 'a, B1: 'a, B2: 'a, T1: 'a, T2: 'a> Emitter<'a, C1, C2, D1, D2, B1, B2, T1, T2> for Decayer<'a, C1, C1, C2, D1, D1, D2, B1, B1, B2, T1, T1, T2>
    where T1: Default,
          C1: SackStorable,
          C2: SackStorable,
          D1: SackStorable,
          D2: SackStorable,
          B1: SackBacker,
          B2: SackBacker,
          T1: TokenLike,
          T2: TokenLike
{
    fn emit(_s1: &'a SackType<C1, D1, B1>) -> (&'a SackType<C1, D1, B1>, &'a SackType<C2, D2, B2>) {
        unimplemented!()
    }
}

impl<'a, C1, C2, C3, C4, D1, D2, D3, D4, B1, B2, B3, B4, T1, T2, T3, T4> Collider<'a, C1, C2, C3, C4, D1, D2, D3, D4, B1, B2, B3, B4, T1, T2, T3, T4>
    for (C1, C2, C3, C4, D1, D2, D3, D4, B1, B2, B3, B4, T1, T2, T3, T4)
    where B1: SackBacker,
          B2: SackBacker,
          B3: SackBacker,
          B4: SackBacker
{
    fn collide(_s1: &'a SackType<C1, D1, B1>, _s2: &'a SackType<C2, D2, B2>) -> (&'a SackType<C3, D3, B3>, &'a SackType<C4, D4, B4>) {
        unimplemented!()
    }
}

// impl<'a, C: 'a, T: 'a> SetStore<'a, BTreeBackedSack<C, (), T>, C> for BTreeBackedSack<C, (), T>
//    where T: TokenLike + Default,
//          C: SackBacker + SackStorable + Default + Ord+Copy
// {
//    fn insert(&'a mut self, value: C) -> &'a Self {
//        //  let (mut b,t) = (self.b,self.t);
//        self.b.insert(value, ());
//        self
//    }
// }

// impl<C, D, T> Default for BTreeBackedSack<C, D, T>
//    where T: Default,
//          C: Default + Ord
// {
//    fn default() -> BTreeBackedSack<C, D, T> {
//        BTreeBackedSack {
//            t: T::default(),
//            b: SackTypes::new(),
//        }
//    }
// }
