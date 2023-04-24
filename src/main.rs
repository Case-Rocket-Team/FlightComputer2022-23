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

    avionics.timer.block_ms(100);

    log::info!("Hello world!");
    
    // Set servos: 0 = 0%, 10_000 = 100%
    // Ex: avionics.servo_output_1.set_turn_off(&avionics.sm, 5_000);
    //     avionics.servo_output_1.set_turn_off(&avionics.sm, 5_000);

    loop {
        // Poll Raven charges
        /*if !avionics.fire_1.is_set() || !avionics.fire_2.is_set() {
            // Charge has gone off!
            log::info!("Fire!")
        }*/

        avionics.servo_output_1.set_turn_off(&avionics.sm, 700);
        avionics.servo_output_2.set_turn_off(&avionics.sm, 700);
        avionics.sm.set_load_ok(&mut avionics.pwm);
        avionics.sm.set_running(&mut avionics.pwm, true);
        avionics.timer.block_ms(2_000);

        avionics.servo_output_1.set_turn_off(&avionics.sm, 1600);
        avionics.servo_output_2.set_turn_off(&avionics.sm, 1600);
        avionics.sm.set_load_ok(&mut avionics.pwm);
        avionics.sm.set_running(&mut avionics.pwm, true);
        avionics.timer.block_ms(2_000);

    }
}