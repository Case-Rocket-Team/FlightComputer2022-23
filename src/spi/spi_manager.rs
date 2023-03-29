use cortex_m::prelude::_embedded_hal_blocking_spi_Transfer;
use embedded_hal::digital::v2::OutputPin;
use imxrt_hal::{self, spi::{SPI}, gpio::{GPIO, Output}};
use teensy4_bsp::t40::*;
use typenum::{UTerm, UInt, B1, B0};
use paste::paste;

type SPIHAL = SPI<UInt<UInt<UInt<UTerm, B1>, B0>, B0>>;

use crate::{spi::{devices::{baro::Baro,flash::W25Q64}}};

macro_rules! gpio_pinout {
    ($($ident: ident $io: tt $pin: ty),+) => {
        $(one_gpio_pin!($ident $io $pin);)+
    }
}

macro_rules! one_gpio_pin {
    ($ident: ident input $pin: ty) => {
        paste! {
            type $ident = $pin;
            type [<$ident Input>] = GPIO<$pin, Input>;
        }
    };
    ($ident: ident output $pin: ty) => {
        paste! {
            type $ident = $pin;
            type [<$ident Output>] = GPIO<$pin, Output>;
        }
    }
}

gpio_pinout! {
    FlashCS output P1,
    BMPCS output P0
}

pub struct SPIInterfaceActiveLow<P: OutputPin> {
    spi_manager: *mut SPIManager,
    pin: P
}

pub trait SPIInterface {
    fn select(&mut self);
    fn deselect(&mut self);
    fn transfer(&self, bytes: &mut [u8]);
}

impl<P: OutputPin> SPIInterface for SPIInterfaceActiveLow<P> {
    fn select(&mut self) {
        let _ = self.pin.set_low();
    }

    fn deselect(&mut self) {
        let _ = self.pin.set_high();
    }

    fn transfer(&self, bytes: &mut [u8]) {
        unsafe {
            let _ = (*self.spi_manager).hal.transfer(bytes);
        }
    }
}

pub trait SPIDevice {
    type TInterface: SPIInterface;
    fn get_interface(&self) -> &Self::TInterface;
    fn new(interface: Self::TInterface) -> Self;
}

#[macro_export]
macro_rules! spi_transfer {
    ($interface: expr, $($val: expr),+) => {
        $interface.select();
        $($interface.transfer($val);)+
        $interface.deselect();
    };
}

#[macro_export]
macro_rules! create_spi_manager {
    ($board: expr, $pins: expr, $spi_pins: tt) => {
        {
            let (_, _, _, spi4_builder) = $board.spi.clock(
                // Handle to CCM
                &mut $board.ccm.handle,
                // See https://www.pjrc.com/teensy/IMXRT1060RM_rev2.pdf
                // Clock at 582 MHz
                imxrt_hal::ccm::spi::ClockSelect::Pll2,
                // Divide above clock speed by 8: 582/8 = 72 Mhz
                imxrt_hal::ccm::spi::PrescalarSelect::LPSPI_PODF_0,
            );
            
            let spi4 = spi4_builder.build(
                $pins.p11, 
                $pins.p12,
                $pins.p13
            );

            SPIManager::new(spi4, SPIPins $spi_pins)
        }
    }
}

macro_rules! spi_devices {
    ($($device: ident $new_type: ident : $type: ty),+) => {
        paste! {
            $(type $new_type = $type;)+

            pub struct SPIManagerDevices {
                $($device: Option<$type>,)+
            }

            pub struct SPIPins {
                $(pub $device: [<$new_type CS>],)+
            }

            $(pub struct [<SPIManagerUsing $new_type>] {
                hal: SPIHAL,
                devices: SPIManagerDevices
            })+

            impl SPIManager {
                $(pub fn [<take_ $device>](mut self) -> ([<SPIManagerUsing $new_type>], $type) {
                    unsafe {
                        let device = self.devices.$device.unwrap_unchecked();
                        self.devices.$device = None;
                        ([<SPIManagerUsing $new_type>] {
                            hal: self.hal,
                            devices: self.devices
                        }, device)
                    }
                })+

                pub fn new(periph: SPIHAL, pins: SPIPins) -> SPIManager {
                    $(
                        let mut [<$device _pin>] = {
                            let mut pin = GPIO::new(pins.$device);
                            pin.set_fast(true);
                            pin.output()
                        };
                        let _ = [<$device _pin>].set_high();                        
                    )+

                    let mut manager = SPIManager {
                        hal: periph,
                        devices: SPIManagerDevices {
                            $($device: None,)+
                        }
                    };

                    $(
                        manager.devices.$device = Some($type::new(SPIInterfaceActiveLow {
                            pin: [<$device _pin>],
                            spi_manager: &mut manager
                        }));
                    )+

                    manager
                }
            }

            $(impl [<SPIManagerUsing $new_type>] {
                pub fn done(mut self, device: $type) -> SPIManager {
                    self.devices.$device = Some(device);
                    SPIManager {
                        hal: self.hal,
                        devices: self.devices
                    }
                }
            })+
        }
    }
}

spi_devices! {
    flash Flash: W25Q64::<SPIInterfaceActiveLow<FlashCSOutput>>,
    baro BMP: Baro::<SPIInterfaceActiveLow<BMPCSOutput>>

}

pub struct SPIManager {
    hal: SPIHAL,
    devices: SPIManagerDevices
}