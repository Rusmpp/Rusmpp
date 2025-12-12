//! Tagged Length Value Field.
//!
//! A Tagged Length Value Field is a special composite field
//! that comprises of three parts:
//! * A 2-octet Integer (Tag)
//!   The tag identifies the parameter.
//!
//! * A 2-octet Integer (Length)
//!   The length field indicates the length of the value
//!   field in octets. Note that this length does not
//!   include the length of the tag and length fields.
//!
//! * An Octet String (Value)
//!   The value field contains the actual data for the
//!   TLV field.
//!
//! The Tag identifies the parameter. The Length indicates
//! the size of the Value field in octets.
//!
//! An example of a TLV is the dest_bearer_type. Its Tag is
//! 0x0007 and has a value size of 1 octet. The value 0x04
//! indicates USSD as a bearer type. In its encoded form,
//! this TLV would appear as follows:
//!
//! 0x0007000104
//!
//! The first 2 octets 0x0007 identifies the Tag
//! dest_bearer_type. The next two octets 0x0001 indicate
//! the 1-octet length of the value field. The value field 0x04
//! indicates USSD.
//!
//! There are two types of NULL encoding for a TLV. The
//! first is a TLV that may not carry a value part. An example
//! of such a TLV is alert_on_message_delivery. This TLV is
//! typically used as an indicator only, i.e. its function is
//! driven by its very presence in the PDU. No data is
//! typically present. However it may carry up to 1 octet of
//! data if required.
//!
//! Here are two examples of how this TLV can be encoded,
//! the first example carries a value, the second example
//! does not:
//!
//! * Tag=0x130C
//! * Length=0x0001
//! * Value=0x01
//!
//! Encoded Format: 0x130C000101
//!
//! * Tag=0x130C
//! * Length=0x0000
//! * Value=NULL
//!
//! Encoded Format: 0x130C0000
//!
//! Note: Only the Tag and Length are encoded. No NULL
//! octets are specified for the zero length Value field.
//!
//! If the TLV itself is not required, then it is not encoded at
//! all. The very absence of the TLV from the PDU is the
//! means by which we set the values to NULL.

mod tag;
pub use tag::*;

pub mod borrowed;
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod owned;
