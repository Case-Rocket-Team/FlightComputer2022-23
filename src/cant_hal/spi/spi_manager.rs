use embedded_hal::digital::v2::OutputPin;

use crate::cant_hal::avionics::SpiDevice;

pub trait SimpleSpiDevice<P: OutputPin>
where Self: SpiDevice {
    fn new(cs: P) -> Self;
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
macro_rules! spi_devices {
    ($($device: ident $new_type: ident : $type: ty)+) => {
        use cortex_m::prelude::_embedded_hal_blocking_spi_Transfer;
        use embedded_hal::digital::v2::OutputPin;
        use imxrt_hal::gpio::Output;
        use paste::paste;
        use imxrt_hal::lpspi::Lpspi;
        use imxrt_hal::iomuxc::Pad;
        use crate::cant_hal::spi::spi_proxy::SpiProxy;

        pub type SpiHal = Lpspi<LpspiPins<Pad<1075806532, 1075807028>, Pad<1075806528, 1075807024>, Pad<1075806536, 1075807032>, Pad<1075806524, 1075807020>>, 4>;

        pub trait SpiDeviceBuilder {
            type TSpiDevice: SpiDevice;
            fn build(self, manager: *mut SpiManager) -> Self::TSpiDevice;
        }

        pub trait SpiDevice {
            fn init(&mut self) {}
        }

        paste! {
            $(pub type $new_type = <$type as SpiDeviceBuilder>::TSpiDevice;)+

            pub struct SpiManagerDevices {
                $(pub $device: <$type as SpiDeviceBuilder>::TSpiDevice,)+
            }

            pub struct SpiDeviceBuilders {
                $(pub $device: $type,)+
            }

            impl<'a> SpiManager {
                pub fn get_spi_hal(&mut self) -> &mut SpiHal {
                    &mut self.spi_hal
                }

                pub fn new(manager: *mut SpiManager, spi_hal: SpiHal, devices: SpiDeviceBuilders) {
                    unsafe {
                        *manager = SpiManager {
                            spi_hal,
                            devices: SpiManagerDevices {
                                $($device: {
                                    let mut device = devices.$device.build(manager);
                                    device.init();
                                    device
                                },)+
                            }
                        };
                    }
                }
            }

            #[repr(C)]
            pub struct SpiManager {
                pub spi_hal: SpiHal,
                pub devices: SpiManagerDevices
            }
        }
    }
}