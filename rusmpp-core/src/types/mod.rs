//! Core `SMPP` types.

mod c_octet_string;
pub use c_octet_string::Error as COctetStringError;

mod empty_or_full_c_octet_string;
pub use empty_or_full_c_octet_string::Error as EmptyOrFullCOctetStringError;

mod any_octet_string;

mod octet_string;
pub use octet_string::Error as OctetStringError;

pub mod borrowed;
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod owned;
pub mod u16;
pub mod u32;
pub mod u8;
