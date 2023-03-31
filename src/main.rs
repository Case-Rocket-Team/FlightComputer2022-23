#![no_std]
#![no_main]

use cortex_m_rt;

use crate::{avionics::{get_avionics}, spi::devices::flash::{WriteDisabled, Ready}};

use teensy4_panic as _;

mod logging;
mod spi;
mod concurrency;
mod avionics;
mod util;
mod layout;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut avionics = get_avionics();
    let (_spi, _flash) = avionics.spi.take_flash();
    let mut flash = _flash.into(WriteDisabled, Ready);

    log::info!("Hello world!");

    let mut write_byte = 0u8;

    loop {
        avionics.timer.block_ms(500);

        let (manu, id) = flash.read_manufacturer_and_device_id();

        log::info!("Found manufacturer {:x?} and device ID {:x?}", manu, id);

        let test_addr = 0x00_00_00;

        flash = flash
                .into_write_enabled()
                .erase_sector(test_addr)
                .into_block_until_ready();

        avionics.timer.block_ms(25);

        flash = flash
                .into_block_until_ready()
                .into_write_enabled()
                .page_program(test_addr, &mut [write_byte])
                .into_block_until_ready();

        avionics.timer.block_ms(25);

        let [read_byte] = flash.read_data::<1>(test_addr);

        log::info!("Wrote {:x?} and read {:x?}!", write_byte, read_byte);

        write_byte += 1;
    }
}