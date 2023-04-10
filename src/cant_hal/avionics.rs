use core::mem::MaybeUninit;
use nb::block;
use teensy4_bsp::board::LpspiPins;
use bsp::board;
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

// Not actually safe to share between threads
unsafe impl Sync for Avionics {}

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
    }, 1_000_000);

    for i in 0..64 {
        loop {
            let mut bytes = [0b01010011u8];
            let res = spi_hal.transfer(&mut bytes);
            match res {
                Ok(_) => break,
                Err(_) => continue
            }
        }
    }

    timer.block_ms(500);
    
    //spi_hal.disabled(|spi| spi.set_clock_hz(board::LPSPI_FREQUENCY, 1_000_000));

    /*lpspi::prepare(&mut pins.p11);
    lpspi::prepare(&mut pins.p12);
    lpspi::prepare(&mut pins.p13);*/

    let spi_manager = unsafe {
        SPI_MANAGER = MaybeUninit::new(SPIManager::new(spi_hal));

        unsafe {
            log::info!("SPI ref addr: {:#?}", SPI_MANAGER);
            log::info!("SPI ptr addr: {:#?}", SPI_MANAGER.as_mut_ptr());
        }

        SPIManager::init(SPI_MANAGER.as_mut_ptr(), SPIDeviceBuilders {
            flash: W25Q64Builder::new(gpio1.output(pins.p1)), 
            radio: Sx127xLoRaBuilder {
                cs: gpio4.output(pins.p2),
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
    