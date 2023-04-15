use embedded_hal::digital::v2::OutputPin;

use crate::cant_hal::{spi::{spi_interface::{SpiInterfaceActiveLow, SpiInterfaceDevice}, spi_proxy::SpiProxy}, avionics::{SpiDeviceBuilder, SpiManager}};

use super::radio::LoRa;

pub struct LoRaBuilder<CS: OutputPin> {
    cs: CS
}

impl<CS: OutputPin> LoRaBuilder<CS> {
    pub fn new(cs: CS) -> Self {
        LoRaBuilder {
            cs
        }
    }
}

impl<CS: OutputPin> SpiDeviceBuilder for LoRaBuilder<CS> {
    type TSpiDevice = LoRa<SpiInterfaceActiveLow<CS>>;
    fn build(self, manager: *mut SpiManager) -> LoRa<SpiInterfaceActiveLow<CS>> {
        LoRa::new(SpiInterfaceActiveLow {
            spi: SpiProxy::new(manager),
            pin: self.cs
        })
    }
}