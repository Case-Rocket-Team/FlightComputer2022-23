#![no_std]
#![no_main]

use cortex_m_rt;
use imxrt_hal::gpio::GPIO;
use teensy4_bsp::pins;

use crate::{avionics::{get_avionics, avionics}, flash::get_flash};

use teensy4_panic as _;

mod logging;
mod flash;
mod concurrency;
mod avionics;
mod util;

#[cortex_m_rt::entry]
fn main() -> ! {
    let avionics = get_avionics();
    let mut flash_cs_pin = GPIO::new(avionics.pins.p1);//pins.p10);
    flash_cs_pin.set_fast(true);
    let flash_cs = flash_cs_pin.output();
    let mut flash = get_flash(flash_cs);

    log::info!("Hello world!");

    /*match spi4.set_clock_speed(bsp::hal::spi::ClockSpeed(SPI_BAUD_RATE_HZ)) {
        Ok(()) => {
            log::info!("Set clock speed to {}Hz", SPI_BAUD_RATE_HZ);
        }
        Err(err) => {
            loop {
                delayer.delay_ms(1_000);
                log::error!("Failed to set clock rate: {:?}", err)
            }
            /*panic!(
                "Unable to set clock speed to {}Hz: {:?}",
                SPI_BAUD_RATE_HZ, err
            );*/
        }
    };*/

    let mut write_byte = 0u8;

    loop {
        avionics.delay(1_000);

        let (manu, id) = flash.read_manufacturer_and_device_id();

        log::info!("Found manufacturer {:x?} and device ID {:x?}", manu, id);

        /*avionics.flash.write_enable();
        avionics.flash.is_busy();*/

        let test_addr = 0x00_00_00;

        flash
                .into_write_enabled()
                .erase_sector(test_addr)
                .into_block_until_ready();

        //avionics.delay(25);

        flash
                .into_block_until_ready()
                .into_write_enabled()
                .page_program(test_addr, &mut [write_byte]);

        //avionics.delay(25);

        /*let [read_byte] = avionics.flash.read_data::<1>(test_addr);

        log::info!("Wrote {:x?} and read {:x?}!", write_byte, read_byte);

        write_byte += 1;*/
    }
}