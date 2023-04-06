use bsp::board;
use crate::cant_hal::spi::devices::flash::W25Q64;
use imxrt_hal::{self, timer::Blocking, pit::Pit};
use teensy4_bsp as bsp;
use crate::{spi_devices, pin_layout};

pub use crate::cant_hal::spi::devices;

pub type Timer = Blocking<Pit<0>, { board::PERCLK_FREQUENCY }>;

pub struct Avionics {
    pub spi: SPIManager,
    pub timer: Timer,
}

pin_layout! {
    P1 FlashCS 
    P2 RadioCS
    P3 RadioReset
}

spi_devices! {
    flash Flash: W25Q64::<SPIInterfaceActiveLow<FlashCS>>
}

pub fn get_avionics() -> Avionics {
    let board::Resources {
        lpspi4,
        pins,
        pit: (pit, _, _, _),
        usb,
        mut gpio1,
        mut gpio2,
        mut gpio3,
        mut gpio4,
        ..
    } = board::t40(board::instances());

    bsp::LoggingFrontend::default_log().register_usb(usb);
    let timer = Timer::from_pit(pit);

    let output = gpio1.output(pins.p1);

    let spi_manager = SPIManager::new(lpspi4, SPIPins {
        flash: output
    });
    
    Avionics {
        spi: spi_manager,
        timer,
    }
}