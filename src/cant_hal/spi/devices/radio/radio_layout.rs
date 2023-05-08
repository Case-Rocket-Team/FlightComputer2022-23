// General traits
pub trait RadioReg {
    const ADDR: u8;
}

pub trait RadioRegPart {
    type Reg: RadioReg;
    type Value: RadioRegPartValue;
    const WRITABLE: bool;
    const READABLE: bool;
    const TRIGGERABLE: bool;
    const CLEARABLE: bool;
    const START: u8;
    const END: u8;
    const PART_MASK: u8;
}

pub trait RadioRegPartValue {
    fn value(&self) -> u8;
}

impl RadioRegPartValue for u8 {
    fn value(&self) -> u8 {
        *self
    }
}