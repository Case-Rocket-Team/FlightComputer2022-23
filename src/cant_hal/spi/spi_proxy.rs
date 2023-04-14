use embedded_hal::blocking::spi::{Transfer, Write};
use imxrt_hal::lpspi;
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

    pub fn block_until_ready(&mut self) {
        log::info!("Starting block until ready...");
        unsafe {
            while (*self.spi_manager).spi_hal.status().intersects(lpspi::Status::BUSY) {}
        }
        log::info!("Block finished");
    }
}

impl Transfer<u8> for SpiProxy {
    type Error = imxrt_hal::lpspi::LpspiError;

    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        unsafe {
            self.block_until_ready();
            (*self.spi_manager).spi_hal.transfer(words);

            Ok(words)
        }
    }
}

impl Write<u8> for SpiProxy {
    type Error = imxrt_hal::lpspi::LpspiError;

    fn write<'w>(&mut self, words: &'w [u8]) -> Result<(), Self::Error> {
        unsafe {
            self.block_until_ready();
            (*self.spi_manager).get_spi_hal().write(words)
        }
    }
}