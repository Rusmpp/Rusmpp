macro_rules! debug_assert_max_sequence_number {
    ($max_sequence_number:expr, $min:expr, $max:expr) => {
        debug_assert!(
            $max_sequence_number >= $min,
            "max_sequence_number must be at least {}, got {}",
            $min,
            $max_sequence_number
        );

        debug_assert!(
            $max_sequence_number <= $max,
            "max_sequence_number must be at most {}, got {}",
            $max,
            $max_sequence_number
        );
    };
}

pub(crate) use debug_assert_max_sequence_number;
