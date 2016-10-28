
use particle::collide::absorb::Absorber;
use sack::{Sack, SackType, SackStorable, SackBacker, SackLike, TokenLike, MultiSack, SackPair};
use std::default::Default;

#[derive(Debug)]
pub struct KVSack<K, V, B, T = ()>(// )>(// )>(
                                   pub Sack<K, V, B, T>)
    where T: TokenLike + SackStorable,
          K: SackStorable + TokenLike,
          V: SackStorable + TokenLike,
          B: SackBacker + Default;




impl<K, V, B, T> SackLike<K, V, Sack<K, V, B, T>> for KVSack<K, V, B, T>
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

impl<K, V, B, T> KVSack<K, V, B, T>
    where T: TokenLike + SackStorable,
          K: SackStorable + TokenLike,
          V: SackStorable + TokenLike,
          B: SackBacker + Default
{
}

impl<K, V, B, T> Default for KVSack<K, V, B, T>
    where T: TokenLike + Default + SackStorable,
          K: SackStorable + TokenLike,
          V: SackStorable + TokenLike,
          B: SackBacker + Default
{
    fn default() -> Self {
        let sack: Sack<K, V, B, T> = Sack {
            t: T::default(),
            b: SackType::Multi(MultiSack::default()),
        };
        KVSack(sack)
    }
}

impl<K, V, B> SackLike<K, V, B> for KVSack<K, V, B>
    where V: SackStorable + TokenLike,
          K: SackStorable + TokenLike,
          B: Default + SackBacker
{
}

impl<K, V, B, T> WriteableKVSack<K, V, B, T> for KVSack<K, V, B, T>
    where K: Ord + Copy,
          B: Default + SackBacker,
          V: Copy + Ord,
          T: TokenLike
{
    fn insert<'a>(s: Box<SackLike<K, V, B>>, k: K, v: V) -> KVSack<K, V, B> {
        unimplemented!();
        // KVSack(Sack{t:(),b:SackType::Multi(MultiSack{b:*Sack::absorb(&s.0,&SackPair(k,v))}))
    }
}

pub trait WriteableKVSack<K, V, B, T>
    where B: SackBacker + Default,
          V: Copy + Ord,
          K: Copy + Ord
{
    fn insert(s1: Box<SackLike<K, V, B>>, k: K, v: V) -> KVSack<K, V, B>;
}
// impl<K, V> From<(K, V)> for KVPair<K, V> {
//    fn from(pair: (K, V)) -> Self {
//        KVPair(pair.0, pair.1)
//    }
// }
