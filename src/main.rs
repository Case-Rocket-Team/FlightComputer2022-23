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

    /*let mut flash = unsafe {
        &mut (*avionics.spi).devices.flash
    };*/
    let mut radio = unsafe {
        &mut (*avionics.spi).devices.radio
    };

    avionics.timer.block_ms(100);

    log::info!("Hello world!");

    /*test_all!{
        flash.test_manufac_and_device_id(),
        flash.test_read_write()
    }*/

    let mut gps = avionics.gps;

    //log::info!("Testing GPS...");

    loop {
        //log::info!("Reading radio ver: {:x}", radio.read_version().ok().unwrap());
        //log::info!("Sending hello world...");
        radio.transmit(b"Hello world!".iter());

        /*log::info!("Receiving...");
        let mut res = ArrayWriterator::<255, u8>::new();
        unsafe {
            while !radio.has_received_packet().unwrap_unchecked() {
                avionics.timer.block_ms(500);
            }
        }
        //radio.read_next_received(&mut res);
        let res_bytes = &res.as_array();
        let str_res = core::str::from_utf8(res_bytes);
        match str_res {
            Ok(str) => log::info!("Received: {}", str),
            Err(_) => {}
        }*/

        //avionics.timer.block_ms(500);
    }
}