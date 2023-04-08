use core::convert::Infallible;

use embedded_hal::digital::v2::OutputPin;

pub struct DummyPin {}

#[allow(unused)]
impl DummyPin {
    pub fn new() -> DummyPin {
        DummyPin {}
    }
}

impl OutputPin for DummyPin {
    type Error = Infallible;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}