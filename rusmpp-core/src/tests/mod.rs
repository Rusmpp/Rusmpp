pub mod borrowed;

#[cfg(feature = "alloc")]
pub mod owned;

/// Trait for creating test instances of a type.
pub trait TestInstance: Sized {
    /// Create test instances of the type.
    fn instances() -> alloc::vec::Vec<Self>;
}
