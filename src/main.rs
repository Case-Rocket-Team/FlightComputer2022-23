#![no_std]
#![no_main]
#![allow(unused, dead_code)]

use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;

use crate::{cant_hal::avionics, cant_hal::avionics::{devices::flash::{WriteDisabled, Ready}, Avionics, take_avionics}, util::ArrayWriterator};

use teensy4_panic as _;
use teensy4_bsp::rt::entry;

mod logging;
mod cant_hal;
mod util;
mod test;

#[entry]
fn main() -> ! {
    let mut avionics = take_avionics();

    let mut flash = unsafe {
        &mut (*avionics.spi).devices.flash
    };
    let mut radio = unsafe {
        &mut (*avionics.spi).devices.radio
    };

    avionics.timer.block_ms(100);

    log::info!("Hello world!");

    test_all!{
        flash.test_manufac_and_device_id(),
        flash.test_read_write()
    }

    let mut gps = avionics.gps;

    loop {
        gps.write(b"PMTK604*6D\r\n");

        let mut bytes = [0u8; 64];
        gps.read(&mut bytes);
        let res = core::str::from_utf8(&bytes);
        match res {
            Ok(msg) => log::info!("{}", msg),
            Err(_) => log::info!("Error reading from GPS")
        }
    }
}