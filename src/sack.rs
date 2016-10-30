use particle::collide::absorb::Absorber;
use particle::collide::collide::Collider;
use std::marker::PhantomData;

pub trait TokenLike: Ord + Copy {}

impl TokenLike for () {}

impl<T> TokenLike for T
    where T: Ord + Copy
{
}

pub trait SackLike<'a, C: 'a, D: 'a, B: 'a>
    where B: SackBacker
{
}


impl<'a, C: 'a, D: 'a, B: 'a> IntoIterator for &'a SackLike<'a, C, D, B> {
    type Item = SackPair<C, D>;
    type IntoIter = SackIter<C, D, B>;
    fn into_iter(self) -> SackIter<C, D, B> {
        unimplemented!()
    }
}

impl<'a, C: 'a, D: 'a, B: 'a, T: 'a> IntoIterator for &'a Sack<C, D, B, T>
    where B: SackBacker
{
    type Item = SackPair<C, D>;
    type IntoIter = SackIter<C, D, B>;
    fn into_iter(self) -> SackIter<C, D, B> {
        unimplemented!()
    }
}

impl<'a, C: 'a, D: 'a, B: 'a> IntoIterator for &'a MultiSack<C, D, B>
    where B: SackBacker
{
    type Item = SackPair<C, D>;
    type IntoIter = SackIter<C, D, B>;
    fn into_iter(self) -> SackIter<C, D, B> {
        unimplemented!()
    }
}

impl<'a, C: 'a, D: 'a> IntoIterator for &'a SackPair<C, D> {
    type Item = (C, D);
    type IntoIter = SingleIter<C, D>;
    fn into_iter(self) -> SingleIter<C, D> {
        unimplemented!()
    }
}

pub struct SingleIter<C, D>(C, D);

impl<C, D> Iterator for SingleIter<C, D> {
    type Item = (C, D);
    fn next(&mut self) -> Option<(C, D)> {}
}

pub struct SackIter<C, D, B> {
    _phantom: PhantomData<(C, D, B)>,
}

impl<C, D, B> Iterator for SackIter<C, D, B> {
    type Item = SackPair<C, D>;
    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

impl<'a, C: 'a> SackLike<'a, C, (), C> for ()
    where C: SackBacker
{
}

impl<'a, C: 'a, D: 'a, B: 'a, T> SackLike<'a, C, D, B> for Sack<C, D, B, T>
    where B: SackBacker
{
}

impl SackStorable for () {}
impl<T> SackStorable for T
    where T: Copy
{
}

pub trait SackStorable: Clone {}

pub trait SackBacker: Clone {}

impl<'a, C, D> SackBacker for (&'a C, &'a D)
    where D: SackStorable,
          C: SackStorable
{
}

#[derive(Debug,Clone)]
pub struct Sack<C, D, B, T> {
    pub t: T,
    pub b: SackType<C, D, B>,
}

// impl<C,D,B,T> Sack<C,D,B,T> {
//    fn new(
// }

impl<'a, C1: 'a, C2: 'a, D1: 'a, D2: 'a, B1: 'a, B2: 'a, T1: 'a, T2: 'a> Absorber<'a, C1, C2, D1, D2, B1, B2, T1, T2> for Sack<C1, D1, B1, T1>
    where B1: SackBacker
{
    fn absorb<S2: SackLike<'a, C2, D2, B2>>(&'a mut self, _s2: S2) -> &'a Self
        where B1: SackBacker,
              B2: SackBacker {
        unimplemented!();
    }
}

impl<'a, C1, C2, C3, C4, D1, D2, D3, D4, B1, B2, B3, B4, T1, T2, T3, T4> Collider<'a, C1, C2, C3, C4, D1, D2, D3, D4, B1, B2, B3, B4, T1, T2, T3, T4> for Sack<C1, D1, B1, T1>
    where B1: SackBacker
{
    fn collide(s1: &'a SackLike<C1, D1, B1>, s2: &'a SackLike<C2, D2, B2>) -> (&'a SackLike<'a, C3, D3, B3>, &'a SackLike<'a, C4, D4, B4>) {
        unimplemented!()
    }
}

#[derive(Debug,Clone)]
pub enum SackType<C, D, B> {
    Empty,
    Single(SackPair<C, D>),
    Multi(MultiSack<C, D, B>),
}

// #[derive(Debug)]
// pub struct EmptySack {}

#[derive(Debug,Clone)]
pub struct SackPair<C, D>(C, D);

impl<C,D> SackPair<C,D>{
    pub fn new(c:C,d:D) -> Self {
        SackPair(c,d)
    }
}

// pub struct SackPair<C, D>(C, D);

impl<'a, C: 'a, D: 'a> SackLike<'a, C, D, (&'a C, &'a D)> for (C, D)
    where C: SackStorable,
          D: SackStorable
{
}
impl<'a, C, D> SackLike<'a, C, D, (&'a C, &'a D)> for (&'a C, &'a D)
    where C: SackStorable,
          D: SackStorable
{
}

#[derive(Debug,Clone)]
pub struct MultiSack<C, D, B> {
    b: B,
    _phantom: PhantomData<(C, D)>,
}

impl<C, D, B> Default for MultiSack<C, D, B>
    where B: SackBacker + Default
{
    fn default() -> Self {
        MultiSack {
            b: B::default(),
            _phantom: PhantomData,
        }
    }
}
