use core::mem::MaybeUninit;
use nb::block;
use teensy4_bsp::board::LpspiPins;
use bsp::{board::{self, LPSPI_FREQUENCY}, pins::t40};
use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use imxrt_hal::{self, timer::Blocking, pit::Pit, iomuxc::lpspi, lpspi::Pins, flexpwm::{Pwm, self}, gpio};
use teensy4_bsp as bsp;
use crate::{spi_devices, pin_layout};

pub use crate::cant_hal::spi::devices;

use self::devices::{flash::W25Q64Builder, radio::{radio::LoRa, builder::LoRaBuilder}};

use super::{dummy_pin::DummyPin, spi::spi_interface::{DefaultSpiInterfaceBuilder, SpiInterfaceActiveLow}};

pub type Timer<const CHAN: u8> = Blocking<Pit<CHAN>, { board::PERCLK_FREQUENCY }>;

pub struct Avionics {
    pub spi: *mut SpiManager,
    pub timer: Timer<0>,
    pub servo_output_1: flexpwm::Output<t40::P7>,
    pub servo_output_2: flexpwm::Output<t40::P8>,
    pub fire_1: gpio::Input<t40::P3>,
    pub fire_2: gpio::Input<t40::P4>,
    pub sm: flexpwm::Submodule<1, 3>,
    pub pwm: flexpwm::Pwm<1>,
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
    radio Radio: LoRaBuilder::<RadioCS>
}

static mut SPI_MANAGER: MaybeUninit<SpiManager> = MaybeUninit::uninit();

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
        mut flexpwm1,
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

    let mut flash_cs = gpio_cs!(gpio1, pins.p1);
    let mut radio_cs = gpio_cs!(gpio4, pins.p2);

    let spi_ptr = unsafe {
        SpiManager::new(SPI_MANAGER.as_mut_ptr(), spi_hal, SpiDeviceBuilders {
            flash: W25Q64Builder::new(flash_cs), 
            radio: LoRaBuilder::new(radio_cs)
        });

        SPI_MANAGER.as_mut_ptr()
    };

    let (mut pwm, mut submods) = flexpwm1;

    let servo_output_2 = flexpwm::Output::new_a(pins.p8);
    let servo_output_1 = flexpwm::Output::new_b(pins.p7);

    let (_, _, _, mut submod3) = submods;


    const PWM_DUTY: i16 = 10_000;

    submod3.set_debug_enable(true);
    submod3.set_wait_enable(true);
    submod3.set_clock_select(flexpwm::ClockSelect::Ipg);
    submod3.set_pair_operation(flexpwm::PairOperation::Independent);
    submod3.set_prescaler(flexpwm::Prescaler::Prescaler128);
    submod3.set_load_frequency(1);
    submod3.set_load_mode(flexpwm::LoadMode::reload_full());
    submod3.set_value(
        flexpwm::FULL_RELOAD_VALUE_REGISTER,
        PWM_DUTY,
    );
    submod3.set_initial_count(&pwm, -PWM_DUTY);

    servo_output_2.set_turn_on(&submod3, 0);
    servo_output_2.set_turn_off(&submod3, 1600);
    servo_output_2.set_output_enable(&mut pwm, true);
    servo_output_1.set_turn_on(&submod3, 0);
    servo_output_1.set_turn_off(&submod3, 1600);
    servo_output_1.set_output_enable(&mut pwm, true);
    submod3.set_load_ok(&mut pwm);
    submod3.set_running(&mut pwm, true);

    let mut pwr1 = gpio4.output(pins.p5);
    pwr1.set_high();
    let mut pwr2 = gpio2.output(pins.p6);
    pwr2.set_high();

    let fire_1 = gpio4.input(pins.p3);
    let fire_2 = gpio4.input(pins.p4);

    Avionics {
        spi: spi_ptr,
        timer,
        servo_output_1,
        servo_output_2,
        fire_1,
        fire_2,
        sm: submod3,
        pwm
    }
}
    