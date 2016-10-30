use sack::{SackType, SackStorable, TokenLike, SackBacker};

/// This is a sack that can decay
pub trait Decayer<'a, C1: 'a, C2: 'a, C3: 'a, D1: 'a, D2: 'a, D3: 'a, B1: 'a, B2: 'a, B3: 'a, T1: 'a, T2: 'a, T3: 'a>
    where C1: SackStorable,
          C2: SackStorable,
          C3: SackStorable,
          D1: SackStorable,
          D2: SackStorable,
          D3: SackStorable,
          T1: TokenLike,
          T2: TokenLike,
          T3: TokenLike,
          B1: SackBacker,
          B2: SackBacker,
          B3: SackBacker
{
    fn decay(&mut self) -> (&'a SackType<C2, D2, B2>, &'a SackType<C3, D3, B3>);
}
