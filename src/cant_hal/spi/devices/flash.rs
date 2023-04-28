use core::marker::PhantomData;
use embedded_hal::digital::v2::OutputPin;

use crate::{cant_hal::{avionics::{SpiManager, SpiDevice, SpiDeviceBuilder}, spi::{spi_proxy::SpiProxy, spi_interface::{SpiInterface, SpiInterfaceActiveLow}}}, util::{Any}, spi_transfer, test::TestResult};

pub struct Ready;
pub struct Busy;

pub struct WriteEnabled;
pub struct WriteDisabled;

pub struct W25Q64<TInterface: SpiInterface> {
    interface: TInterface,
}

impl<P: OutputPin> W25Q64<SpiInterfaceActiveLow<P>> {
    pub fn from_pin(pin: P) -> W25Q64Builder<P> {
        W25Q64Builder { pin }
    }
}

pub struct W25Q64Builder<P: OutputPin> {
    pin: P
}

impl<P: OutputPin> W25Q64Builder<P> {
    pub fn new(cs: P) -> Self {
        W25Q64Builder { pin: cs }
    }
}

impl<P: OutputPin> SpiDeviceBuilder for W25Q64Builder<P> {
    type TSpiDevice = W25Q64<SpiInterfaceActiveLow<P>>;
    fn build(self, spi_manager: *mut SpiManager) -> Self::TSpiDevice {
        W25Q64::new(SpiInterfaceActiveLow {
            spi: SpiProxy::new(spi_manager),
            pin: self.pin
        })
    }
}

impl<I: SpiInterface> SpiDevice for W25Q64<I> {
    fn init(&mut self) {
        self.interface.deselect();
    }
}

impl<TInterface: SpiInterface> W25Q64<TInterface> {
    pub fn new(interface: TInterface) -> Self {
        W25Q64 { interface }
    }
    
    pub fn send_instr(&mut self, bytes: &mut [u8]) {
        spi_transfer!(self.interface, bytes);
    }

    pub fn is_busy(&mut self) -> bool {
        let mut bytes = [0x05, 0x00];
        self.send_instr(&mut bytes);
        (bytes[1] & 0x01) == 1
    }

    pub fn block_until_ready(&mut self) {
        while self.is_busy() {}
    }

    pub fn set_write_enabled(&mut self) {
        self.send_instr(&mut [0x06]);
    }

    pub fn read_manufacturer_and_device_id(&mut self) -> (u8, u8) {
        let mut bytes = [0x90, 0x00, 0x00, 0x00, 0x00, 0x00];
        self.send_instr(&mut bytes);
        (bytes[4], bytes[5])
    }

    pub fn read_data<const TDATALENGTH: usize>(&mut self, addr: u32) -> [u8; TDATALENGTH] {
        let mut part_1 =  [
            0x03_u8, 
            ((addr >> 16) & 0xff) as u8,
            ((addr >> 8) & 0xff) as u8,
            (addr & 0xff) as u8
        ];

        let mut received = [0u8; TDATALENGTH];

        spi_transfer!(self.interface, &mut part_1, &mut received);

        received
    }

    pub fn send_write_instr(&mut self, bytes: &mut [u8]) {
        self.send_instr(bytes);
    }

    pub fn page_program<const TPROGRAMSIZE: usize>(&mut self, addr: u32, data: &mut [u8; TPROGRAMSIZE]){
        let mut part_1 =  [
            0x02_u8, 
            ((addr >> 16) & 0xff) as u8,
            ((addr >> 8) & 0xff) as u8,
            (addr & 0xff) as u8
        ];

        spi_transfer!(self.interface, &mut part_1, data);
    }

    pub fn erase_sector (&mut self, addr: u32) {
        let mut instr =  [
            0x20_u8, 
            ((addr >> 16) & 0xff) as u8,
            ((addr >> 8) & 0xff) as u8,
            (addr & 0xff) as u8
        ];

        self.send_write_instr(&mut instr)
    }

    // -- Tests --

    pub fn test_manufac_and_device_id(&mut self) -> TestResult {
        let (manu, id) = self.read_manufacturer_and_device_id();

        if manu == 0xEF && id == 0x15 {
            TestResult::Pass
        } else {
            log::info!("Found wrong manufacturer and device id: {:x}, {:x}", manu, id);
            TestResult::Fail
        }
    }

    pub fn test_read_write(&mut self) -> TestResult {
        let test_addr = 0x00_00_00;

        self.set_write_enabled();
        self.erase_sector(test_addr);
        self.block_until_ready();

        let write_byte = 0x30;

        self.set_write_enabled();
        self.page_program(test_addr, &mut [write_byte]);
        self.block_until_ready();

        let [read_byte] = self.read_data::<1>(test_addr);
        
        if (write_byte == read_byte) {
            TestResult::Pass
        } else {
            log::info!("Wrong byte read! {:x} vs {:x} expected", read_byte, write_byte);
            TestResult::Fail
        }
    }
}

/*
macro_rules! impl_write_instr {
    ($($fnName:ident (mut $self:ident, $( $arg:ident : $type:ty ),*) $fn:block)+) => {
        $(
            impl<TInterface: SpiInterface> W25Q64State<TInterface, WriteEnabled, Ready> {
                pub fn $fnName (mut $self, $( $arg : $type ),* )
                        -> W25Q64State<TInterface, WriteDisabled, Busy>
                    $fn
            }

            impl<TInterface: SpiInterface> W25Q64State<TInterface, WriteEnabled, Busy> {
                #[doc = "Blocks until ready, then executes "]
                #[doc = stringify!($fnName)]
                pub fn $fnName (mut $self, $( $arg : $type ),* )
                        -> W25Q64State<TInterface, WriteDisabled, Busy>
                    {
                        $self.block_until_ready().$fnName($( $arg ),*)
                    }
            }

            impl<TInterface: SpiInterface> W25Q64State<TInterface, WriteDisabled, Ready> {
                #[doc = "Selects the chip, enables writing, blocks until ready, executes "]
                #[doc = stringify!($fnName)]
                #[doc = " then deselects the chip"]
                pub fn $fnName (mut $self, $( $arg : $type ),* )
                        -> W25Q64<P>
                    {
                        $self.write_enable().$fnName($( $arg ),*)
                    }
            }

            impl<TInterface: SpiInterface> W25Q64State<TInterface, WriteDisabled, Busy> {
                #[doc = "Selects the chip, enables writing, blocks until ready, executes "]
                #[doc = stringify!($fnName)]
                #[doc = " then deselects the chip"]
                pub fn $fnName (mut $self, $( $arg : $type ),* )
                        -> W25Q64<P>
                    {
                        $self.block_until_ready().write_enable().$fnName($( $arg ),*)
                    }
            }
        )+
    };
}*/
/*
impl_write_instr!{
    erase_sector (mut self, addr: u32) {
        let mut instr =  [
            0x20_u8, 
            ((addr >> 16) & 0xff) as u8,
            ((addr >> 8) & 0xff) as u8,
            (addr & 0xff) as u8
        ];

        self.send_write_instr(&mut instr)
    }
}*/