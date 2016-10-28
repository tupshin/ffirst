use particle::collide::absorb::Absorber;
use particle::collide::collide::Collider	;
use std::marker::PhantomData;

pub trait TokenLike: Ord + Copy {}

impl TokenLike for () {}

impl<T> TokenLike for T where T:Ord+Copy {}

pub trait SackLike<C, D, B>
    where B: SackBacker
{
}

impl<C> SackLike<C, (), C> for ()
    where C: SackBacker
{
}

impl <C,D,B,T> SackLike<C,D,B> for Sack<C,D,B,T> where B:SackBacker{}

impl SackStorable for () {}
impl<T> SackStorable for T where T:Copy {}

pub trait SackStorable: Clone {}

pub trait SackBacker: Clone {}

#[derive(Debug,Clone)]
pub struct Sack<C, D, B, T>
    where B: SackBacker
{
    pub t: T,
    pub b: SackType<C, D, B>,
}

//impl<C,D,B,T> Sack<C,D,B,T> {
//    fn new(
//}

impl<S1,S2,C1,C2,D1,D2,B1,B2,T1,T2> Absorber<S1,S2,C1,C2,D1,D2,B1,B2,T1,T2> for Sack<C1,D1,B1,T1> where B1:SackBacker {
    fn absorb<'a>(_s1: &'a SackLike<C1, D1, T1>, _s2: &'a SackLike<C2, D2, T2>) -> &'a SackType<C1, D1, B1> {unimplemented!();}
}

impl<S1,S2,S3,S4,C1,C2,C3,C4,D1,D2,D3,D4,B1,B2,B3,B4,T1,T2,T3,T4> Collider<S1,S2,S3,S4,C1,C2,C3,C4,D1,D2,D3,D4,B1,B2,B3,B4,T1,T2,T3,T4> for Sack<C1,D1,B1,T1> where B1:SackBacker {
    fn collide<'a>(s1: &'a SackLike<C1, D1, B1>, s2: &'a SackLike<C2, D2, B2>) -> (&'a SackLike<C3, D3, B3>, &'a SackLike<C4, D4, B4>){unimplemented!();}
}

#[derive(Debug,Clone)]
pub enum SackType<C, D, B>
    where B: SackBacker
{
    Empty,
    Single(SingleSack<C, D>),
    Multi(MultiSack<C, D, B>),
}

//#[derive(Debug)]
//pub struct EmptySack {}

#[derive(Debug,Clone)]
pub struct SingleSack<C, D> {
    _c: C,
    _d: D,
}

impl<C,D,B> SackLike<C,D,B> for SackPair<C,D> where B:SackBacker {}

pub struct SackPair<C,D>(C,D);

#[derive(Debug,Clone)]
pub struct MultiSack<C, D, B>
    where B: SackBacker
{
    b: B,
    _phantom: PhantomData<(C, D)>,
}

impl<C, D, B> Default for MultiSack<C, D, B>
    where B: SackBacker + Default
{
    fn default() -> Self {
        unimplemented!()
    }
}
