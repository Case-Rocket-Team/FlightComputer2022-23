use embedded_hal::digital::v2::OutputPin;

pub trait SimpleSPIDevice<P: OutputPin> {
    fn new(cs: P) -> Self;
}

#[macro_export]
macro_rules! spi_transfer {
    ($interface: expr, $($val: expr),+) => {
        $interface.select();
        $(let res = $interface.transfer($val);)+
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

        pub trait SPIDeviceBuilder {
            type TSPIDevice: SPIDevice;
            fn build(self, manager: *mut SPIManager) -> Self::TSPIDevice;
        }

        pub trait SPIDevice {
            fn init(&mut self);
        }

        paste! {
            $(pub type $new_type = <$type as SPIDeviceBuilder>::TSPIDevice;)+

            pub struct SPIManagerDevices {
                $(pub $device: <$type as SPIDeviceBuilder>::TSPIDevice,)+
            }

            pub struct SPIDeviceBuilders {
                $(pub $device: $type,)+
            }

            impl<'a> SPIManager {
                pub fn get_spi_hal(&mut self) -> &mut SpiHal {
                    &mut self.spi_hal
                }

                pub fn new(manager: *mut Self, spi_hal: SpiHal,  devices: SPIDeviceBuilders) {
                    unsafe {
                        *manager = SPIManager {
                            spi_hal,
                            devices: SPIManagerDevices {
                                $($device: {
                                    let mut device = devices.$device.build(manager);
                                    device.init();
                                    device
                                },)+
                            }
                        }
                    }
                }
            }

            #[repr(C)]
            pub struct SPIManager {
                pub spi_hal: SpiHal,
                pub devices: SPIManagerDevices
            }
        }
    }
}