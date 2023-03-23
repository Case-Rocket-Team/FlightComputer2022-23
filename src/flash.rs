use embedded_hal::{blocking::spi::Transfer, digital::v2::OutputPin};

use crate::avionics::Avionics;

pub struct Ready;
pub struct Busy;
pub struct WriteEnabled;
pub struct WriteDisabled;


pub struct W25Q64State<TPin: OutputPin, TWritable, TReady> {
    cs: TPin,
    write_enabled: TWritable,
    ready: TReady,
    pub avionics: *mut Avionics
}

pub type W25Q64<P> = W25Q64State<P, WriteDisabled, Ready>;

pub fn get_flash<P: OutputPin>(avionics: *mut Avionics, cs: P) -> W25Q64<P> {
    W25Q64 {
        cs,
        write_enabled: WriteDisabled,
        ready: Ready,
        avionics
    }
}
/*
impl<P: OutputPin> W25Q64<P> {

    pub fn page_program<const TPROGRAMSIZE: usize>(&mut self, addr: u32, mut data: [u8; TPROGRAMSIZE]) {
        let mut part_1 =  [
            0x02_u8, 
            ((addr >> 16) & 0xff) as u8,
            ((addr >> 8) & 0xff) as u8,
            (addr & 0xff) as u8
        ];

        self.write_enable();
        self.block_until_ready();
        self.select();
        avionics().spi.transfer(&mut part_1).ok();
        avionics().spi.transfer(&mut data).ok();
        self.unselect();
    }

    pub fn read_data<const TDATALENGTH: usize>(&mut self, addr: u32) -> [u8; TDATALENGTH] {
        let mut part_1 =  [
            0x03_u8, 
            ((addr >> 16) & 0xff) as u8,
            ((addr >> 8) & 0xff) as u8,
            (addr & 0xff) as u8
        ];

        let mut received = [0u8; TDATALENGTH];

        self.block_until_ready();
        self.select();
        avionics().spi.transfer(&mut part_1).unwrap();
        avionics().spi.transfer(&mut received).unwrap();
        self.unselect();

        received
    }
}*/

macro_rules! transfer_spi {
    ($flash: expr, $($val: expr),+) => {
        unsafe {
            $flash.cs.set_low().unwrap_unchecked();
            $((*($flash.avionics)).spi.transfer($val).unwrap_unchecked();)+
            $flash.cs.set_high().unwrap_unchecked();
        }
    }
}

impl<P: OutputPin, TWritable, TBusy> W25Q64State<P, TWritable, TBusy> {
    pub fn send_instr(&mut self, bytes: &mut [u8]) {
        transfer_spi!(self, bytes);
    }

    pub fn send_instr_set_state<TWriteEnabledAfter, TReady>(mut self, write_enabled_after: TWriteEnabledAfter, ready_after: TReady, bytes: &mut [u8])
            -> W25Q64State<P, TWriteEnabledAfter, TReady> {
        
        transfer_spi!(self, bytes);

        W25Q64State {
            cs: self.cs,
            write_enabled: write_enabled_after,
            ready: ready_after,
            avionics: self.avionics
        }
    }

    pub fn is_busy(&mut self) -> bool {
        let mut bytes = [0x05, 0x00];
        self.send_instr(&mut bytes);
        (bytes[1] & 0x01) == 1
    }

    pub fn into_block_until_ready(mut self)
            -> W25Q64State<P, TWritable, Ready> {
        
        while self.is_busy() {}
        
        W25Q64State {
            cs: self.cs,
            write_enabled: self.write_enabled,
            ready: Ready,
            avionics: self.avionics
        }
    }
}

impl<P: OutputPin, TWritable> W25Q64State<P, TWritable, Ready> {
    pub fn into_write_enabled(self) -> W25Q64State<P, WriteEnabled, Ready> {
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

        transfer_spi!(self, &mut part_1, &mut received);

        received
    }
}

impl<P: OutputPin> W25Q64State<P, WriteEnabled, Ready> {
    pub fn send_write_instr(self, bytes: &mut [u8])
            -> W25Q64State<P, WriteDisabled, Busy> {
        self.send_instr_set_state(WriteDisabled, Busy, bytes)
    }

    pub fn page_program<const TPROGRAMSIZE: usize>(mut self, addr: u32, data: &mut [u8; TPROGRAMSIZE])
            -> W25Q64State<P, WriteDisabled, Busy>{
        let mut part_1 =  [
            0x02_u8, 
            ((addr >> 16) & 0xff) as u8,
            ((addr >> 8) & 0xff) as u8,
            (addr & 0xff) as u8
        ];

        transfer_spi!(self, &mut part_1, data);

        W25Q64State {
            cs: self.cs,
            write_enabled: WriteDisabled,
            ready: Busy,
            avionics: self.avionics
        }
    }

    pub fn erase_sector (self, addr: u32) -> W25Q64State<P, WriteDisabled, Busy> {
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
            impl<P: OutputPin> W25Q64State<P, WriteEnabled, Ready> {
                pub fn $fnName (mut $self, $( $arg : $type ),* )
                        -> W25Q64State<P, WriteDisabled, Busy>
                    $fn
            }

            impl<P: OutputPin> W25Q64State<P, WriteEnabled, Busy> {
                #[doc = "Blocks until ready, then executes "]
                #[doc = stringify!($fnName)]
                pub fn $fnName (mut $self, $( $arg : $type ),* )
                        -> W25Q64State<P, WriteDisabled, Busy>
                    {
                        $self.block_until_ready().$fnName($( $arg ),*)
                    }
            }

            impl<P: OutputPin> W25Q64State<P, WriteDisabled, Ready> {
                #[doc = "Selects the chip, enables writing, blocks until ready, executes "]
                #[doc = stringify!($fnName)]
                #[doc = " then deselects the chip"]
                pub fn $fnName (mut $self, $( $arg : $type ),* )
                        -> W25Q64<P>
                    {
                        $self.write_enable().$fnName($( $arg ),*)
                    }
            }

            impl<P: OutputPin> W25Q64State<P, WriteDisabled, Busy> {
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