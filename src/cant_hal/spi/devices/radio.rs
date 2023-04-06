use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::OutputPin;
use sx127x_lora::LoRa;

use crate::cant_hal::avionics::{SPIManager, SpiHal, Timer};

pub struct Sx127xLoRa<CS: OutputPin, RESET: OutputPin, DELAY: DelayMs<u8>> {
    lora: LoRa<SpiHal, CS, RESET, DELAY>,
}

impl<CS: OutputPin, Reset: OutputPin> Sx127xLoRa<CS, Reset, Timer> {
    fn new(cs: CS, reset: Reset, timer: Timer, spi_manager: &mut SPIManager) {
        //LoRa::new(spi_manager, cs, reset,  915, timer)
    }
}