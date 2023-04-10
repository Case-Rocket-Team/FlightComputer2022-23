use embedded_hal::digital::v2::OutputPin;

pub trait SimpleSPIDevice<P: OutputPin> {
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

        pub type SpiHal = Lpspi<LpspiPins<Pad<1075806532, 1075807028>, Pad<1075806528, 1075807024>, Pad<1075806536, 1075807032>, Pad<1075806524, 1075807020>>, 4>;

        pub struct SPIInterfaceActiveLow<P: OutputPin> {
            pub spi_manager: *mut SPIManager,
            pub pin: P
        }

        pub trait SPIInterface {
            type TCS;
            fn select(&mut self);
            fn deselect(&mut self);
            fn transfer(&self, bytes: &mut [u8]);
        }

        impl<P: OutputPin> SPIInterface for SPIInterfaceActiveLow<P> {
            type TCS = P;
            fn select(&mut self) {
                let _ = self.pin.set_low();
            }

            fn deselect(&mut self) {
                let _ = self.pin.set_high();
            }

            fn transfer(&self, bytes: &mut [u8]) {
                unsafe {
                    let _ = (*self.spi_manager).get_spi_hal().transfer(bytes);
                }
            }
        }

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
                $(pub $device: Option<<$type as SPIDeviceBuilder>::TSPIDevice>,)+
            }

            pub struct SPIDeviceBuilders {
                $(pub $device: $type,)+
            }

            $(
                #[repr(C)]
                pub struct [<SPIManagerUsing $new_type>] {
                    spi_hal: SpiHal,
                    devices: SPIManagerDevices
                }
            )+

            impl<'a> SPIManager {
                $(pub fn [<take_ $device>](mut self) -> ([<SPIManagerUsing $new_type>], $new_type) {
                    unsafe {
                        let device = self.devices.$device.unwrap_unchecked();
                        self.devices.$device = None;
                        // Transmute here to avoid move
                        (core::mem::transmute(self), device)
                    }
                })+

                pub fn get_spi_hal(&mut self) -> &mut SpiHal {
                    &mut self.spi_hal
                }

                pub fn new(spi_hal: SpiHal) -> SPIManager {
                    SPIManager {
                        spi_hal,
                        devices: SPIManagerDevices {
                            $($device: None,)+
                        }
                    }
                }

                pub fn init(manager: *mut Self, devices: SPIDeviceBuilders) {
                    $(unsafe {
                        let mut device = devices.$device.build(manager);
                        device.init();
                        (*manager).devices.$device = Some(device);
                    })+
                }
            }

            $(impl [<SPIManagerUsing $new_type>] {
                pub fn done(mut self, device: $new_type) -> SPIManager {
                    self.devices.$device = Some(device);
                    SPIManager {
                        spi_hal: self.spi_hal,
                        devices: self.devices
                    }
                }
            })+

            #[repr(C)]
            pub struct SPIManager {
                pub spi_hal: SpiHal,
                pub devices: SPIManagerDevices
            }
        }
    }
}