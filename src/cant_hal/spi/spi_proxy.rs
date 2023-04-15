use embedded_hal::blocking::spi::{Transfer, Write};
use imxrt_hal::lpspi::{self, LpspiError};
use crate::cant_hal::avionics::SpiManager;

pub struct SpiProxy {
    spi_manager: *mut SpiManager
}

impl SpiProxy {
    pub fn new(spi_manager: *mut SpiManager) -> Self {
        SpiProxy {
            spi_manager
        }
    }

    /*pub fn block_until_ready(&mut self) {
        log::info!("Starting block until ready...");
        unsafe {
            log::info!("{:#?}", (*self.spi_manager).spi_hal.status());

            while (*self.spi_manager).spi_hal.status().intersects(lpspi::Status::BUSY) {}
        }
        log::info!("Block finished");
    }*/
}

impl Transfer<u8> for SpiProxy {
    type Error = imxrt_hal::lpspi::LpspiError;

    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        unsafe {
            loop {
                let res = (*self.spi_manager).spi_hal.transfer(words);

                // Match statement doesn't work because of the borrow checker...
                if res.is_err() {
                    if res.unwrap_err_unchecked() == LpspiError::Busy {
                        continue;
                    } else {
                        return Err(res.unwrap_err_unchecked());
                    }
                } else {
                    return Ok(words);
                }
            }
        }
    }
}

impl Write<u8> for SpiProxy {
    type Error = imxrt_hal::lpspi::LpspiError;

    fn write<'w>(&mut self, words: &'w [u8]) -> Result<(), Self::Error> {
        unsafe {
            loop {
                let res = (*self.spi_manager).spi_hal.write(words);
                match res {
                    Err(LpspiError::Busy) => continue,
                    _ => break res
                }
            }
        }
    }
}