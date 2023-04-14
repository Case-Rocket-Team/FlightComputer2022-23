pub struct Any;
/*
#[macro_export]
macro_rules! concat_slice {
    ($($slice: expr),*) => {{
        let mut total_len = 0;

        $(total_len += $slice.len();)*

        let new_slice = [0u8; total_len];

        let i = 0;

        $({
            for val in $slice {
                new_slice[i] = val;
                i += 1;
            }
        })*

        new_slice
    }};
} */