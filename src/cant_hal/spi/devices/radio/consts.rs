
#![allow(dead_code)]

/// Modes of the radio and their corresponding register values.
#[derive(Clone, Copy)]
pub enum RadioMode {
    LongRangeMode = 0x80,
    Sleep = 0x00,
    Stdby = 0x01,
    FsTx = 0x02,
    Tx = 0x03,
    RxContinuous = 0x05,
    RxSingle = 0x06,
}

impl RadioMode {
    /// Returns the address of the mode.
    pub fn addr(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Copy)]
pub enum PaConfig {
    PaBoost = 0x80,
    PaOutputRfoPin = 0,
}

#[derive(Clone, Copy)]
pub enum IRQ {
    IrqTxDoneMask = 0x08,
    IrqPayloadCrcErrorMask = 0x20,
    IrqRxDoneMask = 0x40,
}

impl PaConfig {
    pub fn addr(self) -> u8 {
        self as u8
    }
}

impl IRQ {
    pub fn addr(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Copy)]
pub enum FskDataModulationShaping {
    None = 1,
    GaussianBt1d0 = 2,
    GaussianBt0d5 = 10,
    GaussianBt0d3 = 11
}

#[derive(Clone, Copy)]
pub enum FskRampUpRamDown {
    #[allow(non_camel_case_types)]
    _3d4ms = 0b000,
    _2ms = 0b0001,
    _1ms = 0b0010,
    _500us = 0b0011,
    _250us = 0b0100,
    _125us = 0b0101,
    _100us = 0b0110,
    _62us = 0b0111,
    _50us = 0b1000,
    _40us = 0b1001,
    _31us = 0b1010,
    _25us = 0b1011,
    _20us = 0b1100,
    _15us = 0b1101,
    _12us = 0b1110,
    _10us = 0b1111
}