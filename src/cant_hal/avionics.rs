use core::mem::MaybeUninit;

use bsp::board;
use imxrt_hal::iomuxc::lpspi;
use crate::cant_hal::spi::devices::radio::Sx127xLoRaBuilder;
use imxrt_hal::{self, timer::Blocking, pit::Pit};
use teensy4_bsp as bsp;
use crate::{spi_devices, pin_layout};

pub use crate::cant_hal::spi::devices;

use self::devices::flash::W25Q64Builder;

pub type Timer = Blocking<Pit<0>, { board::PERCLK_FREQUENCY }>;

pub struct Avionics {
    pub spi: SPIManager,
    pub timer: Timer,
}

// Not actually safe to share between threads
unsafe impl Sync for Avionics {}

pin_layout! {
    P1 FlashCS 
    P2 RadioCS
    P3 RadioReset
}

spi_devices! {
    flash Flash: W25Q64Builder::<FlashCS>
    radio Radio: Sx127xLoRaBuilder::<RadioCS, RadioReset, Timer>
}

static mut SPI_HAL: MaybeUninit<SpiHal> = MaybeUninit::uninit();

#[allow(unused_variables)]
#[allow(unused_mut)]
pub fn take_avionics() -> Avionics {
    let board::Resources {
        lpspi4,
        mut pins,
        pit: (pit, _, _, _),
        usb,
        mut gpio1,
        mut gpio2,
        mut gpio3,
        mut gpio4,
        ..
    } = board::t40(board::instances());

    bsp::LoggingFrontend::default_log().register_usb(usb);

    let spi_hal =  unsafe {
        SPI_HAL = MaybeUninit::new(Lpspi::without_pins(lpspi4));
        SPI_HAL.assume_init_mut()
    };

    lpspi::prepare(&mut pins.p11);
    lpspi::prepare(&mut pins.p12);
    lpspi::prepare(&mut pins.p13);

    let spi_manager = SPIManager::new(spi_hal);

    Avionics {
        spi: spi_manager,
        timer: Timer::from_pit(pit),
    }
}
    