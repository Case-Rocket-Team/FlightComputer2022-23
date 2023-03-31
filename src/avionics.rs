use core::num::NonZeroU8;

use imxrt_hal::{self, i2c};
use teensy4_bsp as bsp;
use imxrt_hal::gpio::GPIO;
use embedded_hal::digital::v2::OutputPin;

use crate::{spi::{spi_manager::{SPIManager, SPIPins}}, logging::logging, create_spi_manager, i2c::i2c::I2CHAL, uart::uart::UARTHAL};

pub struct Avionics {
    pub spi: SPIManager,
    pub delayer: cortex_m::delay::Delay,
    pub i2c: I2CHAL,
    pub uart: UARTHAL,
}


const I2C_CLOCK_SPEED: i2c::ClockSpeed = i2c::ClockSpeed::KHz400;

macro_rules! deactive_pins {
    ($($pin: expr),+) => {
        $({
            let mut pin = GPIO::new($pin);
            pin.set_fast(true);
            let mut output_pin = pin.output();
            output_pin.set_high();
        })+
    };
}

pub fn get_avionics() -> Avionics {
    let mut board = imxrt_hal::Peripherals::take().unwrap();
    let cortex = cortex_m::Peripherals::take().unwrap();
    //let mut peripherals = bsp::Peripherals::take().unwrap();

    let pins = bsp::pins::t40::from_pads(board.iomuxc);

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
        flash: pins.p1
    });

    deactive_pins!(pins.p0, pins.p2, pins.p9);

    let mut i2c = {
        log::info!("Enabling I2C clocks...");
        let (i2c1_builder, _, i2c3_builder, _) = board.i2c.clock(
            &mut board.ccm.handle,
            bsp::hal::ccm::i2c::ClockSelect::OSC, // 24MHz clock...
            bsp::hal::ccm::i2c::PrescalarSelect::DIVIDE_3, // Divide by 3
        );

        log::info!("Constructing I2C3 instance on pins 16 and 17...");
        //let mut i2c3 = i2c3_builder.build(pins.p16, pins.p17);

        let mut i2c1 = i2c1_builder.build(pins.p19, pins.p18);

        if let Err(err) = i2c1.set_bus_idle_timeout(core::time::Duration::from_micros(200)) {
            log::warn!("Error when setting bus idle timeout: {:?}", err);
        }
        if let Err(err) = i2c1.set_pin_low_timeout(core::time::Duration::from_millis(1)) {
            log::warn!("Error when setting pin low timeout: {:?}", err);
        }
        if let Err(err) = i2c1.set_clock_speed(I2C_CLOCK_SPEED) {
            log::warn!(
                "Error when setting I2C clock speed to {:?}: {:?}",
                I2C_CLOCK_SPEED,
                err
            );
        }

        i2c1
    };

    let uarts = board.uart.clock(
        &mut board.ccm.handle,
        bsp::hal::ccm::uart::ClockSelect::OSC,
        bsp::hal::ccm::uart::PrescalarSelect::DIVIDE_1,
    );

    let mut uart = uarts.uart8.init(pins.p19, pins.p19, 9600).unwrap();
    uart.set_tx_fifo(NonZeroU8::new(4));
    uart.set_rx_fifo(true);
    uart.set_parity(None);
    uart.set_rx_inversion(false);
    uart.set_tx_inversion(false);
        
    Avionics {
        spi: spi_manager,
        delayer: cortex_m::delay::Delay::with_source(
                cortex.SYST, 
                teensy4_bsp::EXT_SYSTICK_HZ,
                cortex_m::peripheral::syst::SystClkSource::External),
        i2c,
        uart
    }
}