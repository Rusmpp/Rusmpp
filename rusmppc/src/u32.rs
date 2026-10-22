/// A `const` version of [`u32::clamp`], that is not yet `const` in stable Rust.
#[inline]
pub const fn u32_clamp(this: u32, min: u32, max: u32) -> u32 {
    if this < min {
        min
    } else if this > max {
        max
    } else {
        this
    }
}

/// Increment a `u32` value by a given increment, wrapping to a specified value if the increment would exceed a maximum value.
#[inline]
pub const fn u32_wrapping_increment(this: u32, increment: u32, max: u32, wrap: u32) -> u32 {
    if this > max - increment {
        wrap
    } else {
        this + increment
    }
}
