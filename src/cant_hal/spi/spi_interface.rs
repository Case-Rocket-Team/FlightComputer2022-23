use embedded_hal::{blocking::spi::{Transfer, Write}, digital::v2::OutputPin};
use imxrt_hal::{iomuxc::flexpwm::Output, lpspi::LpspiError};

use super::spi_proxy::SpiProxy;

pub trait Selectable {
    type Error;
    fn select(&mut self) -> Result<(), Self::Error>;
    fn deselect(&mut self) -> Result<(), Self::Error>;
}

// Set up alias for Transfer<u8> + Write<u8> + Selectable as trait aliases
// aren't supported yet.
pub trait SpiInterface where
Self: Transfer<u8> + Write<u8> + Selectable {}
impl<T: Transfer<u8> + Write<u8> + Selectable> SpiInterface for T {}

pub struct SpiInterfaceActiveLow<P: OutputPin> {
    pub spi: SpiProxy,
    pub pin: P
}

impl<P: OutputPin> SpiInterfaceActiveLow<P> {
    pub fn new(mut pin: P, proxy: SpiProxy) -> SpiInterfaceActiveLow<P> {
        pin.set_low();
        SpiInterfaceActiveLow { spi: proxy, pin }
    }
}

impl<P: OutputPin> Transfer<u8> for SpiInterfaceActiveLow<P> {
    type Error = LpspiError;
    fn transfer<'a>(&mut self, bytes: &'a mut [u8]) -> Result<&'a [u8], <Self as Transfer<u8>>::Error> {
        unsafe {
            self.spi.transfer(bytes)
        }
    }
}

impl<P: OutputPin> Write<u8> for SpiInterfaceActiveLow<P> {
    type Error = LpspiError;
    fn write<'a>(&mut self, bytes: &'a [u8]) -> Result<(), <Self as Transfer<u8>>::Error> {
        unsafe {
            self.spi.write(bytes)
        }
    }
}

impl<P: OutputPin> Selectable for SpiInterfaceActiveLow<P> {
    type Error = <P as embedded_hal::digital::v2::OutputPin>::Error;
    fn select(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low()
    }

    fn deselect(&mut self) -> Result<(), Self::Error> {
        self.pin.set_high()
    }
}