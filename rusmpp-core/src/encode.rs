//! Traits for encoding `SMPP` values.

use crate::Sealed;

/// Trait for determining the length of `SMPP` values.
pub trait Length: Sealed {
    fn length(&self) -> usize;
}

/// Trait for encoding `SMPP` values into a slice.
pub trait Encode: Length + Sealed {
    /// Encode a value into a slice.
    ///
    /// Implementors are allowed to panic if the slice is not big enough to hold the encoded value. If `dst.len()` < [`Length::length`]
    fn encode(&self, dst: &mut [u8]) -> usize;
}

pub(crate) trait EncodeExt: Encode {
    fn encode_move(&self, dst: &mut [u8], size: usize) -> usize {
        size + self.encode(&mut dst[size..])
    }
}

impl<T: Encode> EncodeExt for T {}

impl<T: Sealed> Sealed for Option<T> {}

impl<T: Length> Length for Option<T> {
    fn length(&self) -> usize {
        self.as_ref().map(Length::length).unwrap_or(0)
    }
}

impl<T: Sealed> Sealed for &[T] {}

impl<T: Length> Length for &[T] {
    fn length(&self) -> usize {
        self.iter().map(Length::length).sum()
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<T: Sealed> Sealed for alloc::vec::Vec<T> {}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<T: Length> Length for alloc::vec::Vec<T> {
    fn length(&self) -> usize {
        self.as_slice().length()
    }
}

impl<T: Sealed, const N: usize> Sealed for heapless::vec::Vec<T, N> {}

impl<T: Length, const N: usize> Length for heapless::vec::Vec<T, N> {
    fn length(&self) -> usize {
        self.as_slice().length()
    }
}

impl<T: Encode> Encode for Option<T> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        self.as_ref().map(|item| item.encode(dst)).unwrap_or(0)
    }
}

impl<T: Encode> Encode for &[T] {
    fn encode(&self, dst: &mut [u8]) -> usize {
        self.iter()
            .fold(0, |acc, item| acc + item.encode(&mut dst[acc..]))
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<T: Encode> Encode for alloc::vec::Vec<T> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        self.as_slice().encode(dst)
    }
}

impl<T: Encode, const N: usize> Encode for heapless::vec::Vec<T, N> {
    fn encode(&self, dst: &mut [u8]) -> usize {
        self.as_slice().encode(dst)
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod owned {
    //! Traits for encoding `SMPP` values using a bytes buffer.

    use bytes::BytesMut;

    use crate::Sealed;

    use super::Length;

    /// Trait for encoding `SMPP` values into a buffer.
    pub trait Encode: Length + Sealed {
        /// Encode a value into a destination buffer.
        ///
        /// Implementors are allowed to panic if the slice is not big enough to hold the encoded value. If `dst.capacity()` < [`Length::length`]
        fn encode(&self, dst: &mut BytesMut);
    }

    impl<T: Encode> Encode for Option<T> {
        fn encode(&self, dst: &mut BytesMut) {
            if let Some(item) = self.as_ref() {
                item.encode(dst)
            }
        }
    }

    impl<T: Encode> Encode for &[T] {
        fn encode(&self, dst: &mut BytesMut) {
            for item in *self {
                item.encode(dst);
            }
        }
    }

    impl<T: Encode> Encode for alloc::vec::Vec<T> {
        fn encode(&self, dst: &mut BytesMut) {
            self.as_slice().encode(dst)
        }
    }

    impl<T: Encode, const N: usize> Encode for heapless::vec::Vec<T, N> {
        fn encode(&self, dst: &mut BytesMut) {
            self.as_slice().encode(dst)
        }
    }
}

#[cfg(test)]
mod tests {
    mod borrowed {

        use super::super::*;

        #[test]
        fn length_option() {
            let value: Option<u8> = Some(0u8);
            assert_eq!(value.length(), 1);

            let value: Option<u8> = None;
            assert_eq!(value.length(), 0);
        }

        #[test]
        fn encode_option() {
            let buf = &mut [0u8; 1024];
            let value: Option<u8> = Some(0u8);
            assert!(buf.len() >= value.length());
            let size = value.encode(buf);
            assert_eq!(size, 1);
            assert_eq!(&buf[..size], &[0]);

            let value: Option<u8> = None;
            assert!(buf.len() >= value.length());
            let size = value.encode(buf);
            assert_eq!(size, 0);
        }

        #[cfg(feature = "alloc")]
        mod owned_extensions {
            use crate::types::borrowed::{
                AnyOctetString, COctetString, EmptyOrFullCOctetString, OctetString,
            };

            use super::*;

            #[test]
            fn length_vec() {
                let values: alloc::vec::Vec<u8> = alloc::vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
                assert_eq!(values.length(), 10);

                let values: alloc::vec::Vec<u16> = alloc::vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
                assert_eq!(values.length(), 20);

                let values: alloc::vec::Vec<u32> = alloc::vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
                assert_eq!(values.length(), 40);

                let values =
                    alloc::vec![AnyOctetString::new(b"Hello"), AnyOctetString::new(b"World")];
                assert_eq!(values.length(), 10);

                let values = alloc::vec![
                    COctetString::<1, 6>::new(b"Hello\0").unwrap(),
                    COctetString::<1, 6>::new(b"World\0").unwrap(),
                ];
                assert_eq!(values.length(), 12);

                let values = alloc::vec![
                    EmptyOrFullCOctetString::<6>::new(b"Hello\0").unwrap(),
                    EmptyOrFullCOctetString::<6>::new(b"World\0").unwrap(),
                ];
                assert_eq!(values.length(), 12);

                let values = alloc::vec![
                    OctetString::<0, 5>::new(b"Hello").unwrap(),
                    OctetString::<0, 5>::new(b"World").unwrap(),
                ];
                assert_eq!(values.length(), 10);
            }

            #[test]
            fn encode_vec() {
                let buf = &mut [0u8; 1024];

                let values: alloc::vec::Vec<u8> = alloc::vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
                assert!(buf.len() >= values.length());
                let size = values.encode(buf);
                assert_eq!(size, 10);
                assert_eq!(&buf[..size], &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

                let values: alloc::vec::Vec<u16> = alloc::vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
                assert!(buf.len() >= values.length());
                let size = values.encode(buf);
                assert_eq!(size, 20);
                assert_eq!(
                    &buf[..size],
                    &[0, 0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9]
                );

                let values: alloc::vec::Vec<u32> = alloc::vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
                assert!(buf.len() >= values.length());
                let size = values.encode(buf);
                assert_eq!(size, 40);
                assert_eq!(
                    &buf[..size],
                    &[
                        0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0,
                        0, 0, 6, 0, 0, 0, 7, 0, 0, 0, 8, 0, 0, 0, 9
                    ]
                );

                let values =
                    alloc::vec![AnyOctetString::new(b"Hello"), AnyOctetString::new(b"World")];
                assert!(buf.len() >= values.length());
                let size = values.encode(buf);
                assert_eq!(size, 10);
                assert_eq!(&buf[..size], b"HelloWorld");

                let values = alloc::vec![
                    COctetString::<1, 6>::new(b"Hello\0").unwrap(),
                    COctetString::<1, 6>::new(b"World\0").unwrap(),
                ];
                assert!(buf.len() >= values.length());
                let size = values.encode(buf);
                assert_eq!(size, 12);
                assert_eq!(&buf[..size], b"Hello\0World\0");

                let values = alloc::vec![
                    EmptyOrFullCOctetString::<6>::new(b"Hello\0").unwrap(),
                    EmptyOrFullCOctetString::<6>::new(b"World\0").unwrap(),
                ];
                assert!(buf.len() >= values.length());
                let size = values.encode(buf);
                assert_eq!(size, 12);
                assert_eq!(&buf[..size], b"Hello\0World\0");

                let values = alloc::vec![
                    OctetString::<0, 5>::new(b"Hello").unwrap(),
                    OctetString::<0, 5>::new(b"World").unwrap(),
                ];
                assert!(buf.len() >= values.length());
                let size = values.encode(buf);
                assert_eq!(size, 10);
                assert_eq!(&buf[..size], b"HelloWorld");
            }
        }
    }

    #[cfg(feature = "alloc")]
    mod owned {
        use bytes::BytesMut;

        use crate::types::owned::{
            AnyOctetString, COctetString, EmptyOrFullCOctetString, OctetString,
        };

        use super::super::{Length, owned::Encode};

        #[test]
        fn encode_option() {
            let mut buf = BytesMut::with_capacity(1024);
            let value: Option<u8> = Some(0u8);
            assert!(buf.capacity() >= value.length());
            value.encode(&mut buf);
            assert_eq!(buf.len(), 1);
            assert_eq!(&buf[..1], &[0]);

            let mut buf = BytesMut::with_capacity(1024);
            let value: Option<u8> = None;
            assert!(buf.capacity() >= value.length());
            value.encode(&mut buf);
            assert_eq!(buf.len(), 0);
        }

        #[test]
        fn encode_vec() {
            let mut buf = BytesMut::with_capacity(1024);
            let values: alloc::vec::Vec<u8> = alloc::vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
            assert!(buf.capacity() >= values.length());
            values.encode(&mut buf);
            assert_eq!(buf.len(), 10);
            assert_eq!(&buf[..10], &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

            let mut buf = BytesMut::with_capacity(1024);
            let values: alloc::vec::Vec<u16> = alloc::vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
            assert!(buf.capacity() >= values.length());
            values.encode(&mut buf);
            assert_eq!(buf.len(), 20);
            assert_eq!(
                &buf[..20],
                &[0, 0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9]
            );

            let mut buf = BytesMut::with_capacity(1024);
            let values: alloc::vec::Vec<u32> = alloc::vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
            assert!(buf.capacity() >= values.length());
            values.encode(&mut buf);
            assert_eq!(buf.len(), 40);
            assert_eq!(
                &buf[..40],
                &[
                    0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0,
                    0, 6, 0, 0, 0, 7, 0, 0, 0, 8, 0, 0, 0, 9
                ]
            );

            let mut buf = BytesMut::with_capacity(1024);
            let values = alloc::vec![
                AnyOctetString::from_static_slice(b"Hello"),
                AnyOctetString::from_static_slice(b"World")
            ];
            assert!(buf.capacity() >= values.length());
            values.encode(&mut buf);
            assert_eq!(buf.len(), 10);
            assert_eq!(&buf[..10], b"HelloWorld");

            let mut buf = BytesMut::with_capacity(1024);
            let values = alloc::vec![
                COctetString::<1, 6>::from_static_slice(b"Hello\0").unwrap(),
                COctetString::<1, 6>::from_static_slice(b"World\0").unwrap(),
            ];
            assert!(buf.capacity() >= values.length());
            values.encode(&mut buf);
            assert_eq!(buf.len(), 12);
            assert_eq!(&buf[..12], b"Hello\0World\0");

            let mut buf = BytesMut::with_capacity(1024);
            let values = alloc::vec![
                EmptyOrFullCOctetString::<6>::from_static_slice(b"Hello\0").unwrap(),
                EmptyOrFullCOctetString::<6>::from_static_slice(b"World\0").unwrap(),
            ];
            assert!(buf.capacity() >= values.length());
            values.encode(&mut buf);
            assert_eq!(buf.len(), 12);
            assert_eq!(&buf[..12], b"Hello\0World\0");

            let mut buf = BytesMut::with_capacity(1024);
            let values = alloc::vec![
                OctetString::<0, 5>::from_static_slice(b"Hello").unwrap(),
                OctetString::<0, 5>::from_static_slice(b"World").unwrap(),
            ];
            assert!(buf.capacity() >= values.length());
            values.encode(&mut buf);
            assert_eq!(buf.len(), 10);
            assert_eq!(&buf[..10], b"HelloWorld");
        }
    }
}
