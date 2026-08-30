macro_rules! encode {
    ($ch:expr, $set:expr) => {{
        let mut i = 0;

        while i < $set.len() {
            let (c, byte) = $set[i];

            if c == $ch {
                return Some(byte);
            }

            i += 1;
        }

        None
    }};
}

macro_rules! decode {
    ($byte:expr, $set:expr) => {{
        let mut i = 0;

        while i < $set.len() {
            let (c, b) = $set[i];

            if b == $byte {
                return Some(c);
            }

            i += 1;
        }

        None
    }};
}

pub(super) use decode;
pub(super) use encode;
