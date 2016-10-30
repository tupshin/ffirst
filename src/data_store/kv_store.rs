use particle::collide::absorb::Absorber;
use sack::{Sack, SackStorable, SackBacker, SackType, TokenLike, MultiSack, SackPair};
use std::default::Default;
use universe::BTreeBackedSack;

pub type KVSack<K, V, T = ()> = Sack<K, V>;

impl<K, V, B, T> SackBacker for Sack<K, V, B, T>
    where K: Clone,
          T: Clone,
          B: SackBacker,
          V: Clone
{
}

impl<K: TokenLike, V: SackStorable + Eq + Ord + Clone> TokenLike for Option<(K, V)>
    where V: SackStorable + TokenLike
{
}

impl<K, V, B, T> KVSack<K, V, T, B>
    where T: TokenLike + SackStorable,
          K: SackStorable + TokenLike,
          V: SackStorable + TokenLike,
          B: SackBacker + Default
{
}

impl<K, V, B, T> Default for KVSack<K, V, T, B>
    where T: TokenLike + Default + SackStorable,
          K: SackStorable + TokenLike,
          V: SackStorable + TokenLike,
          B: SackBacker + Default
{
    default fn default() -> Self {
        let sack: Sack<K, V, B, T> = Sack {
            t: T::default(),
            b: SackType::Multi(MultiSack::default()),
        };
        sack
    }
}

impl<K, V, B> Default for KVSack<K, V, (), B>
    where K: SackStorable + TokenLike,
          V: SackStorable + TokenLike,
          B: SackBacker + Default
{
    fn default() -> Self {
        Sack {
            t: (),
            b: SackType::Multi(MultiSack::default()),
        }
    }
}

// impl<'a, K1, K2, K3, K4, V1, V2, V3, V4, B1, B2, B3, B4, T1, T2, T3, T4> Collider<'a, K1, K2, K3, K4, V1, V2, V3, V4, B1, B2, B3, B4, T1, T2, T3, T4> for KVSack<K1, V1, T1, B1>
//    where V1: SackStorable + TokenLike,
//          K1: SackStorable + TokenLike,
//          B1: Default + SackBacker,
//          T1: TokenLike
// {
//    fn collide(s1: &'a SackLike<K1, V1, B1>, s2: &'a SackLike<K2, V2, B2>) -> (&'a SackLike<K3, V3, B3>, &'a SackLike<K4, V4, B4>) {
//        unimplemented!()
//    }
// }


impl<'a, K: 'a, V: 'a, B: 'a, T: 'a> WriteableKVSack<'a, K, V> for KVSack<K, V, T, B>
    where K: SackStorable + TokenLike,
          B: Default + SackBacker,
          V: SackStorable + TokenLike,
          T: TokenLike,
          Sack<K, V, B>: From<(K, V)>
{
    fn insert(self, k: K, v: V) -> Self {
        Sack::absorb(self, (k, v).into())
    }
}

// struct KVPair<K, V>(K, V);

type KVPair<K, V> = SackPair<K, V>;

impl<'a, K: 'a, V: 'a> SackBacker for (K, V)
    where V: Clone,
          K: Clone
{
}

// impl<'a, K1: 'a, K2: 'a, V1: 'a, V2: 'a, B1: 'a, B2: 'a, T1: 'a> Absorber<'a, K1, K2, V1, V2, B1, B2, T1, T1> for KVSack<K1, V1, T1, B1>
//    where K1: Ord + Copy,
//          V1: Ord + Copy,
//          B1: Default + SackBacker,
//          T1: TokenLike
// {
//    default fn absorb(mut self, s2: SackLike<K2, V2, B2>) -> Self
//        where B2: SackBacker {
//        match s2 {
//
//                  }
//        //        for s  in s2.into_iter() {
//        //           // self.absorb(s);
//        //           unimplemented!()
//        //        }
//        self
//    }
// }




pub trait WriteableKVSack<'a, K: 'a, V: 'a>
    where V: SackStorable + TokenLike,
          K: SackStorable + TokenLike
{
    fn insert(mut self, k: K, v: V) -> Self;
}
// impl<K, V> From<(K, V)> for KVPair<K, V> {
//    fn from(pair: (K, V)) -> Self {
//        KVPair(pair.0, pair.1)
//    }
// }
