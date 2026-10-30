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
