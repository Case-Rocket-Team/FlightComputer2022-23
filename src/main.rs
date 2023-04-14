#![no_std]
#![no_main]
#![allow(unused, dead_code)]

use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;

use crate::{cant_hal::avionics, cant_hal::avionics::{devices::flash::{WriteDisabled, Ready}, Avionics, take_avionics}};

use teensy4_panic as _;
use teensy4_bsp::rt::entry;

mod logging;
mod cant_hal;
mod util;
mod test;

#[entry]
fn main() -> ! {
    let mut avionics = take_avionics();

    let mut flash = &mut avionics.spi.devices.flash;
    let mut radio = &mut avionics.spi.devices.radio;

    avionics.timer.block_ms(100);

    log::info!("Hello world!");

    test_all!{
        flash.test_manufac_and_device_id(),
        flash.test_read_write()
    }

    loop {
        radio.hello_world();
        avionics.timer.block_ms(500);
    }
}