use core::convert::Infallible;

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::OutputPin;
use sx127x_lora::LoRa;

use crate::cant_hal::{avionics::{SpiManager, SpiDevice, SpiDeviceBuilder, SpiHal}, spi::{spi_proxy::SpiProxy, spi_interface::SpiInterface}};
pub struct Sx127xLoRaBuilder<CS: OutputPin, RESET: OutputPin, DELAY: DelayMs<u8>> {
    pub cs: CS,
    pub reset: RESET,
    pub delay: DELAY,
}

impl< CS: OutputPin, RESET: OutputPin, DELAY: DelayMs<u8>> SpiDeviceBuilder
        for Sx127xLoRaBuilder<CS, RESET, DELAY> {
    type TSpiDevice = Sx127xLoRa<CS, RESET, DELAY>;

    fn build(self, manager: *mut SpiManager) -> Self::TSpiDevice {
        Sx127xLoRa::new(self.cs, self.reset, self.delay, manager)
    }
}

#[allow(unused)]
pub struct Sx127xLoRa<CS: OutputPin, RESET: OutputPin, DELAY: DelayMs<u8>> {
    lora: LoRa<SpiProxy, CS, RESET, DELAY>,
}

impl<CS: OutputPin, Reset: OutputPin, Delay: DelayMs<u8>> Sx127xLoRa<CS, Reset, Delay> {
    fn new(mut cs: CS, reset: Reset, timer: Delay, spi_manager: *mut SpiManager) -> Self {
        let lora_res = LoRa::new(SpiProxy::new(spi_manager), cs, reset,  915, timer);

        match lora_res {
            Ok(lora) => Sx127xLoRa {
                lora
            },
            Err(e) => {
                match e {
                    sx127x_lora::Error::Reset(ref e) => log::info!("Reset error"),
                    sx127x_lora::Error::CS(ref e) => log::info!("CS error"),
                    sx127x_lora::Error::SPI(ref e) => log::info!("SPI error"),
                    sx127x_lora::Error::Uninformative => log::info!("Uninformative error"),
                    sx127x_lora::Error::Reset(ref e) => log::info!("Reset error"),
                    sx127x_lora::Error::VersionMismatch(ref e) => log::info!("Version error: {}", e),
                    sx127x_lora::Error::Transmitting => log::info!("Transmit error")
                }

                panic!();
            }
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

impl<CS: OutputPin, RESET: OutputPin, DELAY: DelayMs<u8>> SpiDevice
        for Sx127xLoRa<CS, RESET, DELAY> {
    fn init(&mut self) {
        ;
    }
}

/*impl<CS: OutputPin, RESET: OutputPin, DELAY: DelayMs<u8>> SpiInterface
        for Sx127xLoRa<CS, RESET, DELAY> {
    type TCS = CS;
    type TError = Infallible;

    fn select(&mut self) {
        todo!()
    }

    fn deselect(&mut self) {
        todo!()
    }

    fn transfer(&mut self, _bytes: &mut[u8]) -> Result<&[u8], Self::TError> {
        todo!()
    }
}*/