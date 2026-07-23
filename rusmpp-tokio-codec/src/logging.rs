//! Logging utilities.

macro_rules! trace {
    (target: $target:expr, $($arg:tt)+) => {
        #[cfg(feature = "tracing")]
        tracing::trace!(target: $target, $($arg)*);
    };
}

macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: $target, $($arg)*);
    };
}

macro_rules! error {
    (target: $target:expr, $($arg:tt)+) => {
        #[cfg(feature = "tracing")]
        tracing::error!(target: $target, $($arg)*);
    };
}

pub(crate) use debug;
pub(crate) use error;
pub(crate) use trace;
