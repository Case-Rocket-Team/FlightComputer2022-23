#![no_std]
#![no_main]
#![allow(unused, dead_code)]

use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use cortex_m_rt;

use crate::{cant_hal::avionics, cant_hal::avionics::{devices::flash::{WriteDisabled, Ready}, Avionics, take_avionics}};

use teensy4_panic as _;

mod logging;
mod cant_hal;
mod util;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut avionics = take_avionics();

    let (_spi, mut radio) = avionics.spi.take_radio();

    log::info!("Hello world!");

    loop {
        radio.hello_world();
        //radio.read_hello_world();

        avionics.timer.delay_ms(500u16);
    }
}