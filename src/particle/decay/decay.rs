use sack::{SackLike, SackStorable, TokenLike, SackBacker};

/// This is a sack that can decay
pub trait Decayer<S1, S2, S3, C1, C2, C3, D1, D2, D3, B1, B2, B3, T1, T2, T3>
    where S1: SackLike<C1, D1, B1>,
          S2: SackLike<C2, D2, B2>,
          S3: SackLike<C3, D3, B3>,
          C1: SackStorable,
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
    fn decay<'a>(s1: &'a S1) -> (&'a S2, &'a S3);
}
