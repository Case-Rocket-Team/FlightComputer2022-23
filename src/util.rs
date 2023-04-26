pub struct Any;

pub trait Writerator {
    type Item;
    type Error;
    fn push(&mut self, item: Self::Item) -> Result<(), Self::Error>;
}

pub trait SizedWriteratorError {
    fn is_full(&self) -> bool {
        false
    }
}

pub trait SizedWriterator<E: SizedWriteratorError>: Writerator<Error = E> {
    //fn new(size: usize) -> Self;
}

pub enum SizedWriteratorFullError {
    Full
}

impl SizedWriteratorError for SizedWriteratorFullError {
    fn is_full(&self) -> bool {
        true
    }
}

pub struct ArrayWriterator<const SIZE: usize, T> {
    array: [T; SIZE],
    index: usize
}

impl<T, const SIZE: usize> Writerator for ArrayWriterator<SIZE, T> {
    type Item = T;
    type Error = SizedWriteratorFullError;

    fn push(&mut self, val: T) -> Result<(), Self::Error> {
        if self.array.len() > self.index {
            self.array[self.index] = val;
            self.index += 1;
            Ok(())
        } else {
            Err(SizedWriteratorFullError::Full)
        }
    }
}

impl<const SIZE: usize> ArrayWriterator<SIZE, u8> {
    pub fn new() -> Self {
        ArrayWriterator {
            array: [0u8; SIZE],
            index: 0
        }
    }

    pub fn as_array(self) -> [u8; SIZE] {
        self.array
    }
}

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