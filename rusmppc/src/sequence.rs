use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering},
};

#[derive(Clone)]
pub struct SequenceNumber {
    max: u32,
    number: Arc<AtomicU32>,
}

impl SequenceNumber {
    pub fn new(max: u32) -> Self {
        let max = crate::u32::u32_clamp(max, 1, crate::MAX_SEQUENCE_NUMBER);

        Self {
            max,
            number: Arc::new(AtomicU32::new(1)),
        }
    }

    pub fn next(&self) -> u32 {
        self.number
            .update(Ordering::Release, Ordering::Acquire, |number| {
                crate::u32::u32_wrapping_increment(number, 1, self.max, 1)
            })
    }
}
