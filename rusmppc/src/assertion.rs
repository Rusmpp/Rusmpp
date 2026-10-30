macro_rules! assert_min_less_or_equal_to {
    ($min:expr, $max:expr) => {
        assert!(
            $min <= $max,
            "min must be less than or equal to max, got min: {}, max: {}",
            $min,
            $max
        );
    };
}

pub(crate) use assert_min_less_or_equal_to;
