use bsp::board;
use imxrt_hal::{self, timer::Blocking, pit::Pit};
use teensy4_bsp as bsp;

use crate::spi::spi_manager::{SPIManager, SPIPins};

pub type Timer = Blocking<Pit<0>, { board::PERCLK_FREQUENCY }>;

pub struct Avionics {
    pub spi: SPIManager,
    pub timer: Timer,
}

pub fn get_avionics() -> Avionics {
    let board::Resources {
        lpspi4,
        pins,
        pit: (pit, _, _, _),
        usb,
        mut gpio1,
        ..
    } = board::t40(board::instances());

    bsp::LoggingFrontend::default_log().register_usb(usb);
    let timer = Timer::from_pit(pit);
    
    let spi_manager = SPIManager::new(lpspi4, SPIPins {
        flash: gpio1.output(pins.p1)
    });
    
    Avionics {
        spi: spi_manager,
        timer,
    }
}