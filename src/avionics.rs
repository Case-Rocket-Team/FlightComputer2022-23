use embedded_hal::digital::v2::OutputPin;
use imxrt_hal::{self, gpio::GPIO};
use teensy4_bsp as bsp;

use crate::{spi::{spi_manager::{SPIManager, SPIPins}}, logging::logging, create_spi_manager};

pub struct Avionics {
    pub spi: SPIManager,
    pub delayer: cortex_m::delay::Delay,
}

pub fn get_avionics() -> Avionics {
    let mut board = imxrt_hal::Peripherals::take().unwrap();
    let cortex = cortex_m::Peripherals::take().unwrap();

    let pins = bsp::pins::t40::from_pads(board.iomuxc);

    {
        let mut pin = GPIO::new(pins.p2);
        pin.set_fast(true);
        let mut pin_output = pin.output();
        pin_output.set_high();
    }

    // See the `logging` module docs for more info.
    // (Provided by library)
    assert!(logging::init().is_ok());

    // Set the clock speed of the main core to
    // 600 MHz
    board.ccm.pll1.set_arm_clock(
        imxrt_hal::ccm::PLL1::ARM_HZ,
        &mut board.ccm.handle,
        &mut board.dcdc
    );
    
    let spi_manager = create_spi_manager!(board, pins, {
        flash: pins.p1,
        baro: pins.p0
    });
    
    Avionics {
        spi: spi_manager,
        delayer: cortex_m::delay::Delay::with_source(
                cortex.SYST, 
                teensy4_bsp::EXT_SYSTICK_HZ,
                cortex_m::peripheral::syst::SystClkSource::External)
    }
}