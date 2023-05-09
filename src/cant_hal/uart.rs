use embedded_hal::serial::{ Write, Read };

pub trait UartInterface where
Self: Read<u8> + Write<u8> {}
impl<T: Read<u8> + Write<u8>> UartInterface for T {}