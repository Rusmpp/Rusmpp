//! Encoding and decoding support.

pub mod ascii;
pub mod gsm7bit;
pub mod latin1;
pub mod ucs2;

#[cfg(any(test, feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod owned;

pub mod errors;

// TODO: make const when const traits drop

/// A trait for mapping characters before encoding them.
///
/// This can be used to replace unsupported characters with user-defined ones.
pub trait MapChar {
    fn map(&self, c: char) -> char;
}

impl<T: MapChar> MapChar for &T {
    fn map(&self, c: char) -> char {
        (*self).map(c)
    }
}

/// A [`MapChar`] implementation that returns the input character unchanged.
#[derive(Debug)]
pub struct IdentityMapChar;

impl MapChar for IdentityMapChar {
    fn map(&self, c: char) -> char {
        c
    }
}
