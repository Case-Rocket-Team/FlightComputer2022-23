use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::OutputPin;
use sx127x_lora::LoRa;

use crate::cant_hal::avionics::{SPIManager, SpiHal, Timer, SPIDevice, SPIDeviceBuilder, SPIInterface};

pub struct Sx127xLoRaBuilder<CS: OutputPin, RESET: OutputPin, DELAY: DelayMs<u8>> {
    cs: CS,
    reset: RESET,
    delay: DELAY
}

impl<CS: OutputPin, RESET: OutputPin, DELAY: DelayMs<u8>> SPIDeviceBuilder
        for Sx127xLoRaBuilder<CS, RESET, DELAY> {
    type TSPIDevice = Sx127xLoRa<CS, RESET, DELAY>;

    fn build(self, manager: &mut SPIManager) -> Self::TSPIDevice {
        Sx127xLoRa::new(self.cs, self.reset, self.timer, manager)
    }
}

pub struct Sx127xLoRa<CS: OutputPin, RESET: OutputPin, DELAY: DelayMs<u8>> {
    lora: LoRa<&'static mut SPIManager, CS, RESET, DELAY>,
}

impl<CS: OutputPin, Reset: OutputPin> Sx127xLoRa<CS, Reset, Timer> {
    fn new(cs: CS, reset: Reset, timer: Timer, spi_manager: &mut SPIManager) -> Self {
        Sx127xLoRa {
            lora: LoRa::new(spi_manager, cs, reset,  915, timer).ok().unwrap()
        }
    }
}

impl<CS: OutputPin, Reset: OutputPin, > SPIDevice for Sx127xLoRa<CS, Reset, Timer> {

}