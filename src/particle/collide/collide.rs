use sack::SackLike;

/// The most generic sack collision imaginable. Truly anything is possible
/// with enough specialization
pub trait Collider<S1, S2, S3, S4, C1, C2, C3, C4, D1, D2, D3, D4, B1, B2, B3, B4, T1, T2, T3, T4> {
    fn collide<'a>(s1: &'a SackLike<C1, D1, B1>, s2: &'a SackLike<C2, D2, B2>) -> (&'a SackLike<C3, D3, B3>, &'a SackLike<C4, D4, B4>);
}