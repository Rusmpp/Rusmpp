use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering},
};

#[derive(Debug, Clone)]
pub struct SequenceNumber {
    min: u32,
    max: u32,
    number: Arc<AtomicU32>,
}

impl SequenceNumber {
    pub fn new(min: u32, max: u32) -> Self {
        // TODO: check min and max are valid
        Self {
            min,
            max,
            number: Arc::new(AtomicU32::new(min)),
        }
    }

    pub fn next(&self) -> u32 {
        self.number
            .update(Ordering::Release, Ordering::Acquire, |number| {
                crate::u32::u32_wrapping_increment(number, 1, self.max, self.min)
            })
    }
}
