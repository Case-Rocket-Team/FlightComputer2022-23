use cortex_m::prelude::_embedded_hal_blocking_spi_Transfer;
use embedded_hal::digital::v2::OutputPin;
use imxrt_hal::gpio::Output;
use teensy4_bsp::pins::t40::*;
use paste::paste;
use teensy4_bsp::ral;
use imxrt_hal::lpspi::Lpspi;

use crate::{spi::{devices::flash::W25Q64}};

type SpiHal = Lpspi<(), 4>;

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
            let _ = (*self.spi_manager).spi_hal.transfer(bytes);
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

macro_rules! spi_devices {
    ($($device_pin: tt $device: ident $new_type: ident : $type: ty)+) => {
        paste! {
            $(type $new_type = $type;)+
            $(type [<$new_type CS>] = Output<$device_pin>;)+

            pub struct SPIManagerDevices {
                $($device: Option<$type>,)+
            }

            pub struct SPIPins {
                $(pub $device: Output<$device_pin>,)+
            }

            $(pub struct [<SPIManagerUsing $new_type>] {
                spi_hal: SpiHal,
                devices: SPIManagerDevices
            })+

            impl SPIManager {
                $(pub fn [<take_ $device>](mut self) -> ([<SPIManagerUsing $new_type>], $type) {
                    unsafe {
                        let device = self.devices.$device.unwrap_unchecked();
                        self.devices.$device = None;
                        ([<SPIManagerUsing $new_type>] {
                            spi_hal: self.spi_hal,
                            devices: self.devices
                        }, device)
                    }
                })+

                pub fn new(spi_ral: ral::lpspi::LPSPI4, mut pins: SPIPins) -> SPIManager {
                    let spi_hal = SpiHal::without_pins(spi_ral);
                    $(
                        let _ = pins.$device.set_high();                        
                    )+

                    let mut manager = SPIManager {
                        spi_hal,
                        devices: SPIManagerDevices {
                            $($device: None,)+
                        }
                    };

                    $(
                        manager.devices.$device = Some($type::new(SPIInterfaceActiveLow {
                            pin: pins.$device,
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
                        spi_hal: self.spi_hal,
                        devices: self.devices
                    }
                }
            })+
        }
    }
}

spi_devices! {
    P1 flash Flash: W25Q64::<SPIInterfaceActiveLow<FlashCS>>
}

pub struct SPIManager {
    spi_hal: SpiHal,
    devices: SPIManagerDevices
}