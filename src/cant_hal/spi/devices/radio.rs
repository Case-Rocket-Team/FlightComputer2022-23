use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::OutputPin;
use sx127x_lora::LoRa;

use crate::cant_hal::avionics::{SPIManager, SPIDevice, SPIDeviceBuilder, SPIInterface};
pub struct Sx127xLoRaBuilder<CS: OutputPin, RESET: OutputPin, DELAY: DelayMs<u8>> {
    cs: CS,
    reset: RESET,
    delay: DELAY,
}

impl< CS: OutputPin, RESET: OutputPin, DELAY: DelayMs<u8>> SPIDeviceBuilder
        for Sx127xLoRaBuilder<CS, RESET, DELAY> {
    type TSPIDevice = Sx127xLoRa<CS, RESET, DELAY>;

    fn build(self, manager: &'static mut SPIManager) -> Self::TSPIDevice {
        Sx127xLoRa::new(self.cs, self.reset, self.delay, manager)
    }
}

#[allow(unused)]
pub struct Sx127xLoRa<CS: OutputPin, RESET: OutputPin, DELAY: DelayMs<u8>> {
    lora: LoRa<&'static mut SPIManager, CS, RESET, DELAY>,
}

impl<CS: OutputPin, Reset: OutputPin, Delay: DelayMs<u8>> Sx127xLoRa<CS, Reset, Delay> {
    fn new(cs: CS, reset: Reset, timer: Delay, spi_manager: &'static mut SPIManager) -> Self {
        Sx127xLoRa {
            lora: LoRa::new(spi_manager, cs, reset,  915, timer).ok().unwrap()
        }
    }

    pub fn hello_world(&mut self) {
        let mut bytes: [u8; 255] = [0; 255];
        let message = b"Hello World!";
        for (i, byte) in message.iter().enumerate() {
            bytes[i] = *byte;
        }
        self.lora.transmit_payload(bytes, message.len());
    }

    pub fn read_hello_world(&mut self) {
        let poll = self.lora.poll_irq(Some(1_000));
        match poll {
            Ok(size) => {
                let result = self.lora.read_packet().ok().unwrap();
                log::info!("{}", core::str::from_utf8(&result).unwrap());
            }
            Err(_) => {
                log::info!("Poll didn't work");
            }
        }
    }
}

impl<CS: OutputPin, RESET: OutputPin, DELAY: DelayMs<u8>> SPIDevice
        for Sx127xLoRa<CS, RESET, DELAY> {
    fn init(&mut self) {
        todo!()
    }
}

impl<CS: OutputPin, RESET: OutputPin, DELAY: DelayMs<u8>> SPIInterface
        for Sx127xLoRa<CS, RESET, DELAY> {
    type TCS = CS;

    fn select(&mut self) {
        todo!()
    }

    fn deselect(&mut self) {
        todo!()
    }

    fn transfer(&self, _bytes: &mut[u8]) {
        todo!()
    }
}