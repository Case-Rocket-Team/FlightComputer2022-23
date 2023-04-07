use core::marker::PhantomData;

use cortex_m::prelude::_embedded_hal_digital_OutputPin;
use embedded_hal::digital::v2::OutputPin;

use crate::{cant_hal::avionics::{SPIInterface, SPIInterfaceActiveLow, SPIManager, SPIDevice, SPIDeviceBuilder}, util::Any, spi_transfer};

pub struct Ready;
pub struct Busy;

pub struct WriteEnabled;
pub struct WriteDisabled;

pub struct W25Q64State<TInterface: SPIInterface, TWritable, TReady> {
    interface: TInterface,
    write_enabled: TWritable,
    ready: TReady,
}

pub type W25Q64<TInterface> = W25Q64State<TInterface, Any, Any>;

impl<P: OutputPin> W25Q64<SPIInterfaceActiveLow<P>> {
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

impl<P: OutputPin> SPIDeviceBuilder for W25Q64Builder<P> {
    type TSPIDevice = W25Q64<SPIInterfaceActiveLow<P>>;
    fn build(self, spi: &mut SPIManager) -> Self::TSPIDevice {
        W25Q64::create_from_interface(SPIInterfaceActiveLow {
            spi_manager: spi,
            pin: self.pin
        })
    }
}

impl<TInterface: SPIInterface> W25Q64<TInterface> {
    pub fn create_from_interface(interface: TInterface) -> W25Q64<TInterface> {
        W25Q64State {
            interface,
            write_enabled: Any,
            ready: Any,
        }
    }
}

impl<I: SPIInterface> W25Q64<I> {
    fn get_interface(&self) -> &I {
        &self.interface
    }
}

impl<I: SPIInterface> SPIDevice for W25Q64<I> {
    fn init(&mut self) {
        self.interface.deselect();
    }
}

impl<TInterface: SPIInterface, TWritable, TBusy> W25Q64State<TInterface, TWritable, TBusy> {
    pub fn into_any(self) -> W25Q64<TInterface> {
        W25Q64State {
            interface: self.interface,
            write_enabled: Any,
            ready: Any
        }
    }

    pub fn into<TWritable2, TBusy2>(self, writable: TWritable2, busy: TBusy2) -> W25Q64State<TInterface, TWritable2, TBusy2> {
        W25Q64State {
            interface: self.interface,
            write_enabled: writable,
            ready: busy
        }
    }


    pub fn send_instr(&mut self, bytes: &mut [u8]) {
        spi_transfer!(self.interface, bytes);
    }

    pub fn send_instr_set_state<TWriteEnabledAfter, TReady>(mut self, write_enabled_after: TWriteEnabledAfter, ready_after: TReady, bytes: &mut [u8])
            -> W25Q64State<TInterface, TWriteEnabledAfter, TReady> {
        
        spi_transfer!(self.interface, bytes);

        W25Q64State {
            interface: self.interface,
            write_enabled: write_enabled_after,
            ready: ready_after,
        }
    }

    pub fn is_busy(&mut self) -> bool {
        let mut bytes = [0x05, 0x00];
        self.send_instr(&mut bytes);
        (bytes[1] & 0x01) == 1
    }

    pub fn into_block_until_ready(mut self)
            -> W25Q64State<TInterface, TWritable, Ready> {
        
        while self.is_busy() {}
        
        W25Q64State {
            interface: self.interface,
            write_enabled: self.write_enabled,
            ready: Ready,
        }
    }
}

impl<TInterface: SPIInterface, TWritable> W25Q64State<TInterface, TWritable, Ready> {
    pub fn into_write_enabled(self) -> W25Q64State<TInterface, WriteEnabled, Ready> {
        self.send_instr_set_state(WriteEnabled, Ready, &mut [0x06])
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
}

impl<TInterface: SPIInterface> W25Q64State<TInterface, WriteEnabled, Ready> {
    pub fn send_write_instr(self, bytes: &mut [u8])
            -> W25Q64State<TInterface, WriteDisabled, Busy> {
        self.send_instr_set_state(WriteDisabled, Busy, bytes)
    }

    pub fn page_program<const TPROGRAMSIZE: usize>(mut self, addr: u32, data: &mut [u8; TPROGRAMSIZE])
            -> W25Q64State<TInterface, WriteDisabled, Busy>{
        let mut part_1 =  [
            0x02_u8, 
            ((addr >> 16) & 0xff) as u8,
            ((addr >> 8) & 0xff) as u8,
            (addr & 0xff) as u8
        ];

        spi_transfer!(self.interface, &mut part_1, data);

        W25Q64State {
            interface: self.interface,
            write_enabled: WriteDisabled,
            ready: Busy
        }
    }

    pub fn erase_sector (self, addr: u32) -> W25Q64State<TInterface, WriteDisabled, Busy> {
        let mut instr =  [
            0x20_u8, 
            ((addr >> 16) & 0xff) as u8,
            ((addr >> 8) & 0xff) as u8,
            (addr & 0xff) as u8
        ];

        self.send_write_instr(&mut instr)
    }
}

/*
macro_rules! impl_write_instr {
    ($($fnName:ident (mut $self:ident, $( $arg:ident : $type:ty ),*) $fn:block)+) => {
        $(
            impl<TInterface: SPIInterface> W25Q64State<TInterface, WriteEnabled, Ready> {
                pub fn $fnName (mut $self, $( $arg : $type ),* )
                        -> W25Q64State<TInterface, WriteDisabled, Busy>
                    $fn
            }

            impl<TInterface: SPIInterface> W25Q64State<TInterface, WriteEnabled, Busy> {
                #[doc = "Blocks until ready, then executes "]
                #[doc = stringify!($fnName)]
                pub fn $fnName (mut $self, $( $arg : $type ),* )
                        -> W25Q64State<TInterface, WriteDisabled, Busy>
                    {
                        $self.block_until_ready().$fnName($( $arg ),*)
                    }
            }

            impl<TInterface: SPIInterface> W25Q64State<TInterface, WriteDisabled, Ready> {
                #[doc = "Selects the chip, enables writing, blocks until ready, executes "]
                #[doc = stringify!($fnName)]
                #[doc = " then deselects the chip"]
                pub fn $fnName (mut $self, $( $arg : $type ),* )
                        -> W25Q64<P>
                    {
                        $self.write_enable().$fnName($( $arg ),*)
                    }
            }

            impl<TInterface: SPIInterface> W25Q64State<TInterface, WriteDisabled, Busy> {
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