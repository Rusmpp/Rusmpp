//! Encoding/decoding and concatenation support for extra `SMPP` features.

pub mod fallback {
    //! Fallback behavior for encoding/decoding and concatenation.

    pub use rusmpp_extra::fallback::*;
}

pub mod encoding {
    //! Encoding/decoding support.

    pub use rusmpp_extra::encoding::owned::*;

    pub use rusmpp_extra::encoding::EncodeError;

    pub mod gsm7bit {
        //! GSM 7-bit encoding/decoding support.

        pub use rusmpp_extra::encoding::gsm7bit::*;
    }

    pub mod ucs2 {
        //! UCS2 encoding/decoding support.

        pub use rusmpp_extra::encoding::ucs2::*;
    }

    pub mod latin1 {
        //! Latin1 encoding/decoding support.

        pub use rusmpp_extra::encoding::latin1::*;
    }
}

pub mod concatenation {
    //! Concatenation support.

    pub use rusmpp_extra::concatenation::{
        MAX_PARTS, MIN_PARTS, MultipartError, multipart::*, owned::*,
    };
}
