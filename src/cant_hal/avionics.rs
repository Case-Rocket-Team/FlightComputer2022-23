use core::mem::MaybeUninit;
use nb::block;
use teensy4_bsp::board::LpspiPins;
use bsp::board::{self, LPSPI_FREQUENCY};
use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use crate::cant_hal::spi::devices::radio::Sx127xLoRaBuilder;
use imxrt_hal::{self, timer::Blocking, pit::Pit, iomuxc::lpspi, lpspi::Pins};
use teensy4_bsp as bsp;
use crate::{spi_devices, pin_layout};

pub use crate::cant_hal::spi::devices;

use self::devices::flash::W25Q64Builder;

use super::dummy_pin::DummyPin;

pub type Timer<const CHAN: u8> = Blocking<Pit<CHAN>, { board::PERCLK_FREQUENCY }>;

pub struct Avionics {
    pub spi: &'static mut SPIManager,
    pub timer: Timer<0>,
}

macro_rules! gpio_cs {
    ($gpio: expr, $pin: expr) => {{
        let mut output = $gpio.output($pin);
        output.set_high();
        output
    }};
}

pin_layout! {
    P1 FlashCS 
    P2 RadioCS
    P22 RadioReset
}

spi_devices! {
    flash Flash: W25Q64Builder::<FlashCS>
    radio Radio: Sx127xLoRaBuilder::<RadioCS, RadioReset, Timer<1>>
}

static mut SPI_MANAGER: MaybeUninit<SPIManager> = MaybeUninit::uninit();

#[allow(unused_variables)]
#[allow(unused_mut)]
pub fn take_avionics() -> Avionics {
    let board::Resources {
        lpspi4,
        mut pins,
        pit: (pit1, pit2, _, _),
        usb,
        mut gpio1,
        mut gpio2,
        mut gpio3,
        mut gpio4,
        ..
    } = board::t40(board::instances());

    bsp::LoggingFrontend::default_log().register_usb(usb);

    let mut timer = Timer::from_pit(pit1);
    timer.block_ms(500);

    let mut spi_hal = board::lpspi(lpspi4, LpspiPins {
        pcs0: pins.p10,
        sdo: pins.p11,
        sdi: pins.p12,
        sck: pins.p13
    }, LPSPI_FREQUENCY / 16);

    timer.block_ms(500);
    
    //spi_hal.disabled(|spi| spi.set_clock_hz(board::LPSPI_FREQUENCY, 1_000_000));

    /*lpspi::prepare(&mut pins.p11);
    lpspi::prepare(&mut pins.p12);
    lpspi::prepare(&mut pins.p13);*/

    let mut flash_cs = gpio_cs!(gpio1, pins.p1);
    let mut radio_cs = gpio_cs!(gpio4, pins.p2);

    let spi_manager = unsafe {
        let ptr = SPI_MANAGER.as_mut_ptr();
        SPIManager::new(ptr, spi_hal, SPIDeviceBuilders {
            flash: W25Q64Builder::new(flash_cs), 
            radio: Sx127xLoRaBuilder {
                cs: radio_cs,
                reset: gpio1.output(pins.p22),
                delay: Timer::<1>::from_pit(pit2)
            }
        });

        SPI_MANAGER.assume_init_mut() 
    };

    Avionics {
        spi: spi_manager,
        timer,
    }
}
    