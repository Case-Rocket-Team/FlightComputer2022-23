#![no_std]
#![no_main]

use cortex_m_rt;
use imxrt_hal::gpio::GPIO;
use teensy4_bsp::pins;

use crate::{avionics::{get_avionics}, flash::get_flash};

use teensy4_panic as _;

mod logging;
mod flash;
mod concurrency;
mod avionics;
mod util;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut avionics = get_avionics();
    let mut flash = avionics.flash;

    log::info!("Hello world!");

    let mut write_byte = 0u8;

    loop {
        avionics.delayer.delay_ms(500);

        let (manu, id) = flash.read_manufacturer_and_device_id();

        log::info!("Found manufacturer {:x?} and device ID {:x?}", manu, id);

        let test_addr = 0x00_00_00;

        flash = flash
                .into_write_enabled()
                .erase_sector(test_addr)
                .into_block_until_ready();

        avionics.delayer.delay_ms(25);

        flash = flash
                .into_block_until_ready()
                .into_write_enabled()
                .page_program(test_addr, &mut [write_byte])
                .into_block_until_ready();

        avionics.delayer.delay_ms(25);

        let [read_byte] = flash.read_data::<1>(test_addr);

        log::info!("Wrote {:x?} and read {:x?}!", write_byte, read_byte);

        write_byte += 1;
    }
}