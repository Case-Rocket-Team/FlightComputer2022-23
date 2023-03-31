use imxrt_hal::uart::UART;
use typenum::{UInt, UTerm, B1, B0};

pub type UARTHAL = UART<UInt<UInt<UInt<UTerm, B1>, B0>, B0>>;