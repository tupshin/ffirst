use particle::collide::absorb::Absorber;
use particle::collide::collide::Collider;
use sack::{Sack, SackType, SackStorable, SackBacker, SackLike, TokenLike, MultiSack};
use sack::SackPair;
use std::default::Default;
use universe::BTreeBackedSack;

#[derive(Debug)]
pub struct KVSack<K, V, T = (), B = BTreeBackedSack<K, V, T>>(// ), B = BTreeBackedSack<K, V, T>>(// ), B = BTreeBackedSack<K, V, T>>(// ), B = BTreeBackedSack<K, V, T>>(// ), B = BTreeBackedSack<K, V, T>>(// ), B = BTreeBackedSack<K, V, T>>(// ), B = BTreeBackedSack<K, V, T>>(
                                                              pub Sack<K, V, B, T>)
    where T: TokenLike + SackStorable,
          K: SackStorable + TokenLike,
          V: SackStorable + TokenLike,
          B: SackBacker + Default;




impl<'a, K: 'a, V: 'a, B: 'a, T: 'a> SackLike<'a, K, V, Sack<K, V, B, T>> for KVSack<K, V, T, B>
    where T: TokenLike + SackStorable,
          K: TokenLike + SackStorable,
          V: TokenLike + SackStorable,
          B: Default + SackBacker
{
}

impl<K, V, B, T> SackBacker for Sack<K, V, B, T>
    where K: Clone,
          T: Clone,
          B: SackBacker,
          V: Clone
{
}

impl<K: TokenLike, V: SackStorable + Eq + Ord + Clone> TokenLike for Option<(K, V)>
    where V: SackStorable + Copy
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
        KVSack(sack)
    }
}

impl<K, V, B> Default for KVSack<K, V, (), B>
    where K: SackStorable + TokenLike,
          V: SackStorable + TokenLike,
          B: SackBacker + Default
{
    fn default() -> Self {
        let sack: Sack<K, V, B, ()> = Sack {
            t: (),
            b: SackType::Multi(MultiSack::default()),
        };
        KVSack(sack)
    }
}

impl<'a, K: 'a, V: 'a, B: 'a> SackLike<'a, K, V, B> for KVSack<K, V, (), B>
    where V: SackStorable + TokenLike,
          K: SackStorable + TokenLike,
          B: Default + SackBacker
{
}

impl<'a, K1, K2, K3, K4, V1, V2, V3, V4, B1, B2, B3, B4, T1, T2, T3, T4> Collider<'a, K1, K2, K3, K4, V1, V2, V3, V4, B1, B2, B3, B4, T1, T2, T3, T4> for KVSack<K1, V1, T1, B1>
    where V1: SackStorable + TokenLike,
          K1: SackStorable + TokenLike,
          B1: Default + SackBacker,
          T1: TokenLike
{
    fn collide(s1: &'a SackLike<K1, V1, B1>, s2: &'a SackLike<K2, V2, B2>) -> (&'a SackLike<'a, K3, V3, B3>, &'a SackLike<'a, K4, V4, B4>) {
        unimplemented!()
    }
}


impl<'a, K: 'a, V: 'a, B: 'a, T> WriteableKVSack<'a, K, V> for KVSack<K, V, T, B>
    where K: Ord + Copy,
          B: Default + SackBacker,
          V: Copy + Ord,
          T: TokenLike
{
    fn insert(&'a mut self, k: K, v: V) -> &'a Self {
        <Self as Absorber<'a, K, K, V, V, B, B, T, T>>::absorb(self, SackPair::new(k, v))
        //       <'a, K, K, V, V, B, B, T,T>
        // unimplemented!()

    }
}

// struct KVPair<K, V>(K, V);

type KVPair<K, V> = SackPair<K, V>;

impl<'a, K: 'a, V: 'a, B: 'a> SackLike<'a, K, V, B> for SackPair<K, V>
    where V: Clone,
          K: Clone,
          B: SackBacker
{
}

impl<'a, K: 'a, V: 'a> SackBacker for (K, V)
    where V: Clone,
          K: Clone
{
}

impl<'a, K1: 'a, K2: 'a, V1: 'a, V2: 'a, B1: 'a, B2: 'a, T1: 'a> Absorber<'a, K1, K2, V1, V2, B1, B2, T1, T1> for KVSack<K1, V1, T1, B1>
    where K1: Ord + Copy,
          V1: Ord + Copy,
          B1: Default + SackBacker,
          T1: TokenLike
{
    default fn absorb<S2>(&'a mut self, s2: S2) -> &'a Self
        where B2: SackBacker,
              S2: SackLike<'a, K2, V2, B2> {
        match s2 {

                  }
        //        for s  in s2.into_iter() {
        //           // self.absorb(s);
        //           unimplemented!()
        //        }
        self
    }
}

pub trait WriteableKVSack<'a, K: 'a, V: 'a>
    where V: Copy + Ord,
          K: Copy + Ord
{
    fn insert(&'a mut self, k: K, v: V) -> &'a Self;
}
// impl<K, V> From<(K, V)> for KVPair<K, V> {
//    fn from(pair: (K, V)) -> Self {
//        KVPair(pair.0, pair.1)
//    }
// }
