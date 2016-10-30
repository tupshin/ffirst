use particle::collide::absorb::Absorber;
use particle::collide::collide::Collider;
use std::marker::PhantomData;

pub trait TokenLike: Ord + Clone {}

impl TokenLike for () {}

impl<T> TokenLike for T
    where T: Ord + Clone
{
}

impl<'a, C: 'a, D: 'a, B: 'a> IntoIterator for &'a SackType<C, D, B>
    where B: SackBacker
{
    type Item = (C, D);
    type IntoIter = SackIter<C, D, B>;
    fn into_iter(self) -> SackIter<C, D, B> {
        unimplemented!()
    }
}

impl<'a, C: 'a, D: 'a, B: 'a, T: 'a> IntoIterator for Sack<C, D, B, T>
    where B: SackBacker
{
    type Item = (C, D);
    type IntoIter = SackIter<C, D, B>;
    fn into_iter(self) -> SackIter<C, D, B> {
        unimplemented!()
    }
}

impl<'a, C: 'a, D: 'a, B: 'a> IntoIterator for &'a MultiSack<C, D, B>
    where B: SackBacker
{
    type Item = (C, D);
    type IntoIter = SackIter<C, D, B>;
    fn into_iter(self) -> SackIter<C, D, B> {
        unimplemented!()
    }
}

pub struct SingleIter<C, D>(C, D);

impl<C, D> Iterator for SingleIter<C, D> {
    type Item = (C, D);
    fn next(&mut self) -> Option<(C, D)> {
        unimplemented!()
    }
}

pub struct SackIter<C, D, B> {
    _phantom: PhantomData<(C, D, B)>,
}

impl<C, D, B> Iterator for SackIter<C, D, B> {
    type Item = (C, D);
    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

// impl<'a, C: 'a> SackLike<'a, C, (), C> for ()
//    where C: SackBacker
// {
// }

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

impl SackBacker for () {}

#[derive(Debug,Clone)]
pub struct Sack<C, D, B, T = ()> {
    pub t: T,
    pub b: SackType<C, D, B>,
}



impl<C, D, B, T> Default for Sack<C, D, B, T>
    where B: Default,
          T: Default
{
    default fn default() -> Self {
        Sack {
            b: SackType::Multi(MultiSack {
                b: B::default(),
                _phantom: PhantomData,
            }),
            t: T::default(),
        }
    }
}
 impl<C,D,B,T> Sack<C,D,B,T> where B:SackBacker,T:Default{
    fn new(s:(C,D)) -> Self {
        Sack{t:T::default(),b:SackType::Single(s.0,s.1)}
    }
 }

// impl<'a, C1: 'a, D1: 'a, T1: 'a> Absorber<'a, C1, C1, D1, D1, (C1,D1), (C1,D1), T1, T1> for Sack<C1, D1, (C1,D1), T1> where C1:SackStorable,D1:SackStorable{
//    fn absorb(mut self, s2: Sack<C1, D1,(C1,D1) >) -> Self {
//        match self.b {
//            SackType::Single(c, d) => unimplemented!(),
//            SackType::Multi(s) => unimplemented!(),
//            SackType::Empty => Sack { t: self.t, b: s2.b },
//        }
//    }
// }
impl<'a, C1: 'a, D1: 'a, B1: 'a, T1: 'a> Absorber<'a, C1, C1, D1, D1, B1, B1, T1> for Sack<C1, D1, B1, T1>
    where B1: SackBacker,
          C1: SackStorable,
          D1: SackStorable,
{
    default fn absorb(s1: Sack<C1, D1, B1, T1>, s2: Sack<C1, D1, B1>) -> Self {
        let t = s1.t;
        match s1.b {
            SackType::Single(c, d) => unimplemented!(),
            SackType::Multi(b) => {
                Sack {
                    t: t,
                    b: SackType::Multi(MultiSack::absorb(Sack {
                                                             t: (),
                                                             b: SackType::Multi(b),
                                                         },
                                                         s2)),
                }
            }
            SackType::Empty => Sack{t:t,b:s2.b},
        }
    }
}


// impl<'a, C1: 'a, D1: 'a, B1: 'a, T1: 'a> Absorber<'a, C1, C1, D1, D1, B1, (C1,D1), T1, T1> for Sack<C1, D1, B1, T1>
//    where B1: SackBacker,C1:SackStorable,D1:SackStorable
// {
//    fn absorb(mut self, s2: Sack<C1, D1, (C1,D1)>) -> Self
//        where B1: SackBacker {
//        match self.b {
//            SackType::Single(c, d) => unimplemented!(),
//            SackType::Multi(s) => {
//             for s in self.b {
//                 self.absorb(Sack::from(s));
//             };
//             self
//            },
//            SackType::Empty =>  self.absorb(s2),
//        }
//    }
// }

// impl<'a, C1: 'a, D1: 'a, B1: 'a, T1: 'a> Absorber<'a, C1, C1, D1, D1, B1, B1, T1, T1> for Sack<C1, D1, B1, T1>
//    where B1: SackBacker
// {
//    default fn absorb(mut self, s2: Sack<C1, D1, (C1, D1)>) -> Self
//        where B1: SackBacker {
//        match self.b {
//            SackType::Single(c, d) => unimplemented!(),
//            SackType::Multi(s) => {
//                for s in self.b {
//                    self.absorb(Sack::from(s));
//                }
//                self
//            }
//            SackType::Empty => self.absorb(s2),
//        }
//    }
// }


// impl<'a, C1: 'a, C2: 'a, D1: 'a, D2: 'a, B1: 'a, B2: 'a, T1: 'a, T2: 'a> Absorber<'a, C1, C2, D1, D2, B1, B2, T1, T2> for Sack<C1, D1, B1, T1>
//    where B1: SackBacker,
//          B2: SackBacker
// {
//     fn absorb(mut self, s2: Sack<C2, D2, B2>) -> Self
//        where B1: SackBacker,
//              B2: SackBacker {
//        unimplemented!()
//    }
// }

//impl<C, D, B> From<(C, D)> for Sack<C, D, B> where B:SackBacker{
//    default fn from(pair: (C, D)) -> Self {
//       Sack::new(pair)
//    }
//}

impl<C, D, B> Into<Sack<C, D, B, ()>> for SackType<C, D, B> {
    fn into(self) -> Sack<C, D, B, ()> {
        match self {
            SackType::Empty=>{unimplemented!()}
            SackType::Single(c,d)=>{unimplemented!()}
            SackType::Multi(b)=>{unimplemented!()}
        }
    }
}


impl<'a, C1, C2, C3, C4, D1, D2, D3, D4, B1, B2, B3, B4, T1, T2, T3, T4> Collider<'a, C1, C2, C3, C4, D1, D2, D3, D4, B1, B2, B3, B4, T1, T2, T3, T4> for Sack<C1, D1, B1, T1>
    where B1: SackBacker,
          B2: SackBacker,
          B3: SackBacker,
          B4: SackBacker
{
    fn collide(_s1: &'a SackType<C1, D1, B1>, _s2: &'a SackType<C2, D2, B2>) -> (&'a SackType<C3, D3, B3>, &'a SackType<C4, D4, B4>) {
        unimplemented!();
    }
}

#[derive(Debug,Clone)]
pub enum SackType<C, D, B> {
    Empty,
    Single(C, D),
    Multi(MultiSack<C, D, B>),
}

impl<C, D, B> IntoIterator for SackType<C, D, B> {
    type Item = (C, D);
    type IntoIter = SackIter<C, D, (C, D)>;
    fn into_iter(self) -> SackIter<C, D, (C, D)> {
        unimplemented!()
    }
}

// #[derive(Debug)]
// pub struct EmptySack {}

pub type SackPair<C, D> = Sack<C, D, (C, D)>;

impl<C, D> From<(C, D)> for Sack<C, D, (C, D), ()> {
    fn from(pair: (C, D)) -> Self {
        Sack {
            t: (),
            b: (SackType::Single(pair.0, pair.1)),
        }
    }
}

// pub struct SackPair<C, D>(C, D);

// impl<'a, C, D> SackLike<'a, C, D, (&'a C, &'a D)> for (&'a C, &'a D)
//    where C: SackStorable,
//          D: SackStorable
// {
// }

#[derive(Debug,Clone)]
pub struct MultiSack<C, D, B> {
    b: B,
    _phantom: PhantomData<(C, D)>,
}

impl<'a, C1: 'a, C2: 'a, D1: 'a, D2: 'a, B1: 'a, B2: 'a, T1: 'a> Absorber<'a, C1, C2, D1, D2, B1, B2, T1> for MultiSack<C1, D1, B1>
    where B2: SackBacker,
          B1: SackBacker,
          MultiSack<C1, D1, B1>: Collider<'a, C1, C2, C1, (), D1, D2, D1, (), B1, B2, B1, (), T1, (), T1, ()>
{
    fn absorb(s1: Sack<C1, D1, B1, T1>, s2: Sack<C2, D2, B2>) -> Self {
        unimplemented!();
    }
}


impl<'a, C1: 'a, C2: 'a, C3: 'a, C4: 'a, D1: 'a, D2: 'a, D3: 'a, D4: 'a, B1: 'a, B2: 'a, B3: 'a, B4: 'a> Collider<'a,
                                                                                                                  C1,
                                                                                                                  C2,
                                                                                                                  C3,
                                                                                                                  C4,
                                                                                                                  D1,
                                                                                                                  D2,
                                                                                                                  D3,
                                                                                                                  D4,
                                                                                                                  B1,
                                                                                                                  B2,
                                                                                                                  B3,
                                                                                                                  B4,
                                                                                                                  (),
                                                                                                                  (),
                                                                                                                  (),
                                                                                                                  ()>
    for MultiSack<C1, D1, B1>
    where B2: SackBacker,
          B1: SackBacker,
          B3: SackBacker,
          B4: SackBacker
{
    fn collide(_s1: &'a SackType<C1, D1, B1>, _s2: &'a SackType<C2, D2, B2>) -> (&'a SackType<C3, D3, B3>, &'a SackType<C4, D4, B4>) {
        unimplemented!();
    }
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

impl<C, D, B> Iterator for MultiSack<C, D, B> {
    type Item = (C, D);
    fn next(&mut self) -> Option<(C, D)> {
        unimplemented!()
    }
}
