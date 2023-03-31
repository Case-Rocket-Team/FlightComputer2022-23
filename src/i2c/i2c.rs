use cortex_m::prelude::{_embedded_hal_blocking_i2c_Read, _embedded_hal_blocking_i2c_Write};
use imxrt_hal::i2c::I2C;
use typenum::{UInt, UTerm, B1};

pub type I2CHAL = I2C<UInt<UTerm, B1>>;

pub fn scan_i2c(i2c: &mut I2CHAL) {
    log::info!("Starting scan...");
    let mut i = 0;
    for address in 1..127 {
        if i2c.read(address, &mut [0u8; 1]).is_ok() {
            log::info!("Found device on {:x}", address);
            i += 1;
        }
    }
    log::info!("Found {} device(s)", i)
}