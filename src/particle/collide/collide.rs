use sack::{SackType, SackBacker};

/// The most generic sack collision imaginable. Truly anything is possible
/// with enough specialization
pub trait Collider<'a, C1, C2, C3, C4, D1, D2, D3, D4, B1, B2, B3, B4, T1, T2, T3, T4>
    where B1: SackBacker,
          B2: SackBacker,
          B3: SackBacker,
          B4: SackBacker
{
    fn collide(s1: &'a SackType<C1, D1, B1>, s2: &'a SackType<C2, D2, B2>) -> (&'a SackType<C3, D3, B3>, &'a SackType<C4, D4, B4>);
}
