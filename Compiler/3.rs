use std::ops::{Add, Sub};
use std::fmt::{Debug, Formatter, Result};

pub trait Pos {
    fn from_usize(n: usize) -> Self;
    fn to_usize(&self) -> usize;
    fn from_u32(n: u32) -> Self;
    fn to_u32(&self) -> u32;
}

macro_rules! impl_pos {
    (
        $(
            $(#[$attr:meta])*
            $vis:vis struct $ident:ident($inner_vis:vis $inner_ty:ty);
        )*
    ) => {
        $(
            $(#[$attr])*
            $vis struct $ident($inner_vis $inner_ty);

            impl Pos for $ident {
                #[inline(always)]
                fn from_usize(n: usize) -> $ident {
                    $ident(n as $inner_ty)
                }

                #[inline(always)]
                fn to_usize(&self) -> usize {
                    self.0 as usize
                }

                #[inline(always)]
                fn from_u32(n: u32) -> $ident {
                    $ident(n as $inner_ty)
                }

                #[inline(always)]
                fn to_u32(&self) -> u32 {
                    self.0 as u32
                }
            }

            impl Add for $ident {
                type Output = $ident;

                #[inline(always)]
                fn add(self, rhs: $ident) -> $ident {
                    $ident(self.0 + rhs.0)
                }
            }

            impl Sub for $ident {
                type Output = $ident;

                #[inline(always)]
                fn sub(self, rhs: $ident) -> $ident {
                    $ident(self.0 - rhs.0)
                }
            }
        )*
    };
}

impl_pos! {
    /// A byte offset.
    ///
    /// Keep this small (currently 32-bits), as AST contains a lot of them.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct BytePos(pub u32);

    /// A character offset.
    ///
    /// Because of multibyte UTF-8 characters, a byte offset
    /// is not equivalent to a character offset. The [`SourceMap`] will convert [`BytePos`]
    /// values to `CharPos` values as necessary.
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct CharPos(pub usize);
}

impl Debug for BytePos {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "BytePos({})", self.0)
    }
}

impl Debug for CharPos {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "CharPos({})", self.0)
    }
}

fn main() {
    let byte_pos1 = BytePos::from_usize(100);
    let byte_pos2 = BytePos::from_u32(50);
    let sum_byte_pos = byte_pos1 + byte_pos2;
    println!("Sum of byte_pos1 and byte_pos2: {:?}", sum_byte_pos);

    let char_pos1 = CharPos::from_usize(200);
    let char_pos2 = CharPos::from_u32(100);
    let diff_char_pos = char_pos1 - char_pos2;
    println!("Difference of char_pos1 and char_pos2: {:?}", diff_char_pos);
}
