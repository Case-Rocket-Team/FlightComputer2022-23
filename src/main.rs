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

    /*let mut radio = match avionics.spi.devices.radio.as_mut() {
        Some(r) => r,
        None => loop {
            log::info!("There is no radio!");
            avionics.timer.block_ms(500);
        }
    };*/

    let mut flash = avionics.spi.devices.flash.as_mut().unwrap();
    log::info!("Hello world!");

    let mut write_byte = 0u8;

    loop {
        avionics.timer.block_ms(500);

        let (manu, id) = flash.read_manufacturer_and_device_id();

        log::info!("Found manufacturer {:x?} and device ID {:x?}", manu, id);

        let test_addr = 0x00_00_00;

        flash.set_write_enabled();
        flash.erase_sector(test_addr);
        flash.block_until_ready();

        avionics.timer.block_ms(25);

        flash.block_until_ready();
        flash.set_write_enabled();
        flash.page_program(test_addr, &mut [write_byte]);
        flash.block_until_ready();

        avionics.timer.block_ms(25);

        let [read_byte] = flash.read_data::<1>(test_addr);

        log::info!("Wrote {:x?} and read {:x?}!", write_byte, read_byte);

        write_byte += 1;
    }
}