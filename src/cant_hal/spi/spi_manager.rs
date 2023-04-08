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

        pub type SpiHal = Lpspi<(), 4>;

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
            fn build(self, manager: &'static mut SPIManager) -> Self::TSPIDevice;
        }

        pub trait SPIDevice {
            fn init(&mut self);
        }

        paste! {
            $(pub type $new_type = <$type as SPIDeviceBuilder>::TSPIDevice;)+

            pub struct SPIManagerDevices {
                $($device: Option<<$type as SPIDeviceBuilder>::TSPIDevice>,)+
            }

            pub struct SPIDeviceBuilders {
                $(pub $device: $type,)+
            }

            $(
                #[repr(C)]
                pub struct [<SPIManagerUsing $new_type>] {
                    spi_hal: &'static mut SpiHal,
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
                    self.spi_hal
                }

                pub fn new(spi_hal: &'static mut SpiHal) -> SPIManager {
                    SPIManager {
                        spi_hal,
                        devices: SPIManagerDevices {
                            $($device: None,)+
                        }
                    }
                }

                pub fn init(&'static mut self, devices: SPIDeviceBuilders) {
                    $({
                        let self_ptr: *mut Self = self;
                        let self_ref_clone: &'static mut Self = unsafe {
                            self_ptr.as_mut().unwrap()
                        };
                        let mut device = devices.$device.build(self_ref_clone);
                        device.init();
                        self.devices.$device = Some(device);
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
                pub spi_hal: &'static mut SpiHal,
                pub devices: SPIManagerDevices
            }

            impl<'a> embedded_hal::blocking::spi::Transfer<u8> for &'a mut SPIManager {
                type Error = imxrt_hal::lpspi::LpspiError;

                fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
                    self.get_spi_hal().transfer(words)
                }
            }

            impl<'a> embedded_hal::blocking::spi::Write<u8> for &'a mut SPIManager {
                type Error = imxrt_hal::lpspi::LpspiError;

                fn write<'w>(&mut self, words: &'w [u8]) -> Result<(), Self::Error> {
                    self.get_spi_hal().write(words)
                }
            }
        }
    }
}