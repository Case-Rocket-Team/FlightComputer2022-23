use cortex_m::prelude::_embedded_hal_digital_OutputPin;
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
        use teensy4_bsp::ral;
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
                    let _ = (*self.spi_manager).spi_hal.transfer(bytes);
                }
            }
        }

        pub trait SPIDeviceBuilder {
            type TSPIDevice: SPIDevice;
            fn build(self, manager: &mut SPIManager) -> Self::TSPIDevice;
        }

        pub trait SPIDevice {
            type TInterface: SPIInterface;
            fn get_interface(&self) -> &Self::TInterface;
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

            $(pub struct [<SPIManagerUsing $new_type>] {
                spi_hal: SpiHal,
                devices: SPIManagerDevices
            })+

            impl SPIManager {
                $(pub fn [<take_ $device>](mut self) -> ([<SPIManagerUsing $new_type>], $new_type) {
                    unsafe {
                        let device = self.devices.$device.unwrap_unchecked();
                        self.devices.$device = None;
                        ([<SPIManagerUsing $new_type>] {
                            spi_hal: self.spi_hal,
                            devices: self.devices
                        }, device)
                    }
                })+

                pub fn new(spi_ral: ral::lpspi::LPSPI4, mut devices: SPIDeviceBuilders) -> SPIManager {
                    let spi_hal = SpiHal::without_pins(spi_ral);
                    let mut manager = SPIManager {
                        spi_hal,
                        devices: SPIManagerDevices {
                            $($device: None,)+
                        }
                    };

                    $({
                        let mut device = devices.$device.build(&mut manager);
                        device.init();
                        manager.devices.$device = Some(device);
                    })+

                    manager
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

            pub struct SPIManager {
                spi_hal: SpiHal,
                devices: SPIManagerDevices
            }
        }
    }
}