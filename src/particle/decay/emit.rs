use particle::decay::decay::Decayer;
use sack::{SackType, SackStorable, SackBacker, TokenLike};

pub trait Emitter<'a, C1: 'a, C2: 'a, D1: 'a, D2: 'a, B1: 'a, B2: 'a, T1: 'a, T2: 'a>
    : Decayer<'a, C1, C1, C2, D1, D1, D2, B1, B1, B2, T1, T1, T2>
    where C1: SackStorable,
          C2: SackStorable,
          D1: SackStorable,
          D2: SackStorable,
          B1: SackBacker,
          B2: SackBacker,
          T1: TokenLike,
          T2: TokenLike
{
    fn emit(s1: &'a SackType<C1, D1, B1>) -> (&'a SackType<C1, D1, B1>, &'a SackType<C2, D2, B2>);
}

// impl<S1, S2, C1, C2, D1, D2, B1, B2, T1, T2> Emitter<S1, S2, C1, C2, D1, D2, B1, B2, T1, T2> for S1
//    where //T1: Default
//          S1: SackLike<C1, D1>+Decayer<S1, S1, S2,C1, C1, C2,D1, D1, D2,B1, B1, B2,T1, T1,T2>,
//          S2: SackLike<C2, D2>,C1:SackStorable,C2:SackStorable,D1:SackStorable,D2:SackStorable
// {
//    fn emit<'a>(_s1: &'a S1) -> (&'a S1, &'a S2);
// }
