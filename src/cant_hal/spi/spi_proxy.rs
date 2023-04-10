use embedded_hal::blocking::spi::{Transfer, Write};
use crate::cant_hal::avionics::SPIManager;

pub struct SpiProxy {
    spi_manager: *mut SPIManager
}

impl SpiProxy {
    pub fn new(spi_manager: *mut SPIManager) -> Self {
        SpiProxy {
            spi_manager
        }
    }
}

impl Transfer<u8> for SpiProxy {
    type Error = imxrt_hal::lpspi::LpspiError;

    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        log::info!("Transfer thru proxy:");
        for word in words.iter() {
            log::info!(" 0x{:0>2x}", word);
        }
        log::info!("SPI addr: {:#?}", self.spi_manager);
        unsafe {
            (*self.spi_manager).spi_hal.transfer(words)
        }
    }
}

impl Write<u8> for SpiProxy {
    type Error = imxrt_hal::lpspi::LpspiError;

    fn write<'w>(&mut self, words: &'w [u8]) -> Result<(), Self::Error> {
        unsafe {
            (*self.spi_manager).get_spi_hal().write(words)
        }
    }
}