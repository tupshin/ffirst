use particle::collide::collide::Collider;
use sack::{SackLike,SackType,SackBacker};

/// An important special case of a Collider is where a sack is absorbed into another one without
/// changing the type signature of the original
pub trait Absorber<S1, S2, C1, C2,D1, D2, B1, B2,T1, T2>
    : Collider<S1, S2, S1, (), C1, C2, C1, (), D1, D2, D1, (), B1, B2, B1, (), T1, T2, T1, ()> {
    fn absorb<'a>(_s1: &'a SackLike<C1, D1, T1>, _s2: &'a SackLike<C2, D2, T2>) ->&'a SackType<C1, D1, B1> where B1:SackBacker;
}
