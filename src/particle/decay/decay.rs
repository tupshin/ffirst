use sack::{SackLike, SackStorable, TokenLike, SackBacker};

/// This is a sack that can decay
pub trait Decayer<'a, S1: 'a, S2: 'a, S3: 'a, C1: 'a, C2: 'a, C3: 'a, D1: 'a, D2: 'a, D3: 'a, B1: 'a, B2: 'a, B3: 'a, T1: 'a, T2: 'a, T3: 'a>
    
    where S1: SackLike<'a, C1, D1, B1>,
          S2: SackLike<'a, C2, D2, B2>,
          S3: SackLike<'a, C3, D3, B3>,
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
    fn decay(s1: &'a S1) -> (&'a S2, &'a S3);
}
