use bsp::board::{Resources, self};
use imxrt_hal::{self, timer::Blocking, pit::Pit};
use teensy4_bsp as bsp;

use crate::{spi::{spi_manager::{SPIManager, SPIPins}}};

pub struct Avionics {
    pub spi: SPIManager,
    pub timer: Blocking<Pit<0>, { board::PERCLK_FREQUENCY }>
}

pub fn get_avionics() -> Avionics {
    let board::Resources {
        lpspi4,
        pins,
        pit: (pit, _, _, _),
        usb,
        mut gpio1,
        gpio2,
        gpio3,
        gpio4,
        ..
    } = board::t40(board::instances());

    bsp::LoggingFrontend::default_log().register_usb(usb);
    let mut timer = Blocking::<_, { board::PERCLK_FREQUENCY }>::from_pit(pit);
    
    let spi_manager = SPIManager::new(lpspi4, SPIPins {
        flash: gpio1.output(pins.p1)
    });
    
    Avionics {
        spi: spi_manager,
        timer,
    }
}