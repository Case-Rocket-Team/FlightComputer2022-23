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

#[entry]
fn main() -> ! {
    let mut avionics = take_avionics();

    let mut radio = match avionics.spi.devices.radio.as_mut() {
        Some(r) => r,
        None => loop {
            log::info!("There is no radio!");
            avionics.timer.block_ms(500);
        }
    };

    log::info!("Hello world!");

    loop {
        log::info!("Ran loop!");

        //radio.hello_world();
        //radio.read_hello_world();

        avionics.timer.delay_ms(500u16);
    }
}