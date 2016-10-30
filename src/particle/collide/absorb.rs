use particle::collide::collide::Collider;
use sack::{SackType, SackBacker, Sack};

/// An important special case of a Collider is where a sack is absorbed into another one without
/// changing the type signature of the original
pub trait Absorber<'a, C1: 'a, C2: 'a, D1: 'a, D2: 'a, B1: 'a, B2: 'a, T1: 'a>
    : Collider<'a, C1, C2, C1, (), D1, D2, D1, (), B1, B2, B1, (), T1, (), T1, ()>
    where B1: SackBacker,
          B2: SackBacker
{
    fn absorb(s1: Sack<C1, D1, B1, T1>, s2: Sack<C2, D2, B2>) -> Self;
}
