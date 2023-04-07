use bsp::board;
use crate::cant_hal::spi::devices::flash::W25Q64;
use crate::cant_hal::spi::devices::radio::Sx127xLoRa;
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

pin_layout! {
    P1 FlashCS 
    P2 RadioCS
    P3 RadioReset
}

spi_devices! {
    flash Flash: W25Q64Builder::<FlashCS>
    //radio Radio: Sx127xLoRaBuilder::<RadioCS, RadioReset, Timer>
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

    let spi_manager = SPIManager::new(lpspi4, SPIDeviceBuilders {
        flash: W25Q64Builder::new(gpio1.output(pins.p1)),
        //radio: gpio1.output(pins.p2),
    });
    
    Avionics {
        spi: spi_manager,
        timer,
    }
}