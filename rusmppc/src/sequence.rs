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
    /// # Panics
    ///
    /// Panics if `min` is greater than `max`.
    pub fn new(min: u32, max: u32) -> Self {
        crate::assertion::assert_min_less_or_equal_to!(min, max);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequence_numbers_wrap_around_correctly() {
        let sequence_number = SequenceNumber::new(1, 10);

        for i in 0..20 {
            assert_eq!(sequence_number.next(), i % 10 + 1);
        }
    }

    #[test]
    fn sequence_numbers_wrap_to_min_not_zero() {
        let sequence_number = SequenceNumber::new(5, 7);

        let got: Vec<u32> = (0..7).map(|_| sequence_number.next()).collect();

        assert_eq!(got, vec![5, 6, 7, 5, 6, 7, 5]);
    }
}
