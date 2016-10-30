use particle::collide::collide::Collider;
use sack::{SackLike, SackType, SackBacker};

/// An important special case of a Collider is where a sack is absorbed into another one without
/// changing the type signature of the original
pub trait Absorber<'a, C1: 'a, C2: 'a, D1: 'a, D2: 'a, B1: 'a, B2: 'a, T1: 'a, T2: 'a>
    : Collider<'a, C1, C2, C1, (), D1, D2, D1, (), B1, B2, B1, (), T1, T2, T1, ()> {
    fn absorb<S2 /* : */>(&'a mut self, _s2: S2) -> &'a Self
        where B2: SackBacker,
              S2: SackLike<'a, C2, D2, B2>;
}
