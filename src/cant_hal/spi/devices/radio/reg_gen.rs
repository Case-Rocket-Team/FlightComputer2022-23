
// GENERATED FILE, DO NOT EDIT!

use crate::cant_hal::spi::devices::radio::radio_layout::*; 

// Register
pub struct RegFifo;

impl RadioReg for RegFifo {
    const ADDR: u8 = 0x00;
}
// Register part
/// LoRa base-band FIFO data input/output. FIFO is cleared an not
/// accessible when device is in SLEEPmode
pub struct RegFifoFifo;

impl RadioRegPart for RegFifoFifo {
    type Reg = RegFifo;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
}



// Register
pub struct RegOpMode;

impl RadioReg for RegOpMode {
    const ADDR: u8 = 0x01;
}
// Register part
///
/// 0 --> FSK/OOK Mode
///
/// 1 --> LoRa Mode
/// This bit can be modified only in Sleep mode. A write operation on
/// other device modes is ignored.
pub struct RegOpModeLongRangeMode;

impl RadioRegPart for RegOpModeLongRangeMode {
    type Reg = RegOpMode;
    type Value = LongRangeModeValue;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 7;
}


// Register enum
pub enum LongRangeModeValue {
        
        /// FSK/OOK Mode
        FskOrOokMode,
    
        /// LoRa Mode
        LoRaMode,
    }

impl RadioRegPartValue for LongRangeModeValue {
    fn value(&self) -> u8 {
        match self {
           Self::FskOrOokMode => 0b0,
           Self::LoRaMode => 0b1
        }
    }
}
// Register part
/// This bit operates when device is in Lora mode; if set it allows
/// access to FSK registers page located in address space
/// (0x0D:0x3F) while in LoRa mode
///
/// 0 --> Access LoRa registers page 0x0D: 0x3F
///
/// 1 --> Access FSK registers page (in mode LoRa) 0x0D: 0x3F
pub struct RegOpModeAccessSharedReg;

impl RadioRegPart for RegOpModeAccessSharedReg {
    type Reg = RegOpMode;
    type Value = AccessSharedRegValue;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 6;
    const END: u8 = 6;
}


// Register enum
pub enum AccessSharedRegValue {
        
        /// Access LoRa registers page 0x0D: 0x3F
        AccessLoRaRegistersPage0x0D_0x3F,
    
        /// Access FSK registers page (in mode LoRa) 0x0D: 0x3F
        AccessFskRegistersPage0x0D_0x3F,
    }

impl RadioRegPartValue for AccessSharedRegValue {
    fn value(&self) -> u8 {
        match self {
           Self::AccessLoRaRegistersPage0x0D_0x3F => 0b0,
           Self::AccessFskRegistersPage0x0D_0x3F => 0b1
        }
    }
}
// Register part
/// Access Low Frequency Mode registers
///
/// 0 --> High Frequency Mode (access to HF test registers)
///
/// 1 --> Low Frequency Mode (access to LF test registers)
pub struct RegOpModeLowFrequencyModeOn;

impl RadioRegPart for RegOpModeLowFrequencyModeOn {
    type Reg = RegOpMode;
    type Value = LowFrequencyModeOnValue;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 3;
    const END: u8 = 3;
}


// Register enum
pub enum LowFrequencyModeOnValue {
        
        /// High Frequency Mode (access to HF test registers)
        HighFrequencyMode,
    
        /// Low Frequency Mode (access to LF test registers)
        LowFrequencyMode,
    }

impl RadioRegPartValue for LowFrequencyModeOnValue {
    fn value(&self) -> u8 {
        match self {
           Self::HighFrequencyMode => 0b0,
           Self::LowFrequencyMode => 0b1
        }
    }
}
// Register part
/// Device modes
///
/// 000 --> SLEEP
///
/// 001 --> STDBY
///
/// 010 --> Frequency synthesis TX (FSTX)
///
/// 011 --> Transmit (TX)
///
/// 100 --> Frequency synthesis RX (FSRX)
///
/// 101 --> Receive continuous (RXCONTINUOUS)
///
/// 110 --> receive single (RXSINGLE)
///
/// 111 --> Channel activity detection (CAD)
pub struct RegOpModeMode;

impl RadioRegPart for RegOpModeMode {
    type Reg = RegOpMode;
    type Value = ModeValue;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = true;
    const CLEARABLE: bool = false;
    const START: u8 = 2;
    const END: u8 = 0;
}


// Register enum
pub enum ModeValue {
        
        /// SLEEP
        Sleep,
    
        /// STDBY
        Stdby,
    
        /// Frequency synthesis TX (FSTX)
        FsTx,
    
        /// Transmit (TX)
        Tx,
    
        /// Frequency synthesis RX (FSRX)
        FsRx,
    
        /// Receive continuous (RXCONTINUOUS)
        RxContinuous,
    
        /// receive single (RXSINGLE)
        RxSingle,
    
        /// Channel activity detection (CAD)
        Cad,
    }

impl RadioRegPartValue for ModeValue {
    fn value(&self) -> u8 {
        match self {
           Self::Sleep => 0b000,
           Self::Stdby => 0b001,
           Self::FsTx => 0b010,
           Self::Tx => 0b011,
           Self::FsRx => 0b100,
           Self::RxContinuous => 0b101,
           Self::RxSingle => 0b110,
           Self::Cad => 0b111
        }
    }
}
// Register
pub struct RegFrMsb;

impl RadioReg for RegFrMsb {
    const ADDR: u8 = 0x06;
}
// Register part
/// MSB of RF carrier frequency
pub struct RegFrMsbFrf;

impl RadioRegPart for RegFrMsbFrf {
    type Reg = RegFrMsb;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
}



// Register
pub struct RegFrMid;

impl RadioReg for RegFrMid {
    const ADDR: u8 = 0x07;
}
// Register part
/// MSB of RF carrier frequency
/// 
pub struct RegFrMidFrf;

impl RadioRegPart for RegFrMidFrf {
    type Reg = RegFrMid;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
}



// Register
pub struct RegFrLsb;

impl RadioReg for RegFrLsb {
    const ADDR: u8 = 0x08;
}
// Register part
/// NEEDS FIX LSB of RF carrier frequency
/// Resolution is 61.035 Hz if F(XOSC) = 32 MHz. Default value is
/// 0x6c8000 = 434 MHz. Register values must be modified only when
/// device is in SLEEP or STAND-BY mode.
/// for RF blocks
pub struct RegFrLsbFrf;

impl RadioRegPart for RegFrLsbFrf {
    type Reg = RegFrLsb;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = true;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
}



// Register
pub struct RegPaConfig;

impl RadioReg for RegPaConfig {
    const ADDR: u8 = 0x09;
}
// Register part
/// Selects PA output pin
///
/// 0 --> RFO pin. Output power is limited to +14 dBm.
///
/// 1 --> PA_BOOST pin. Output power is limited to +20 dBm
pub struct RegPaConfigPaSelect;

impl RadioRegPart for RegPaConfigPaSelect {
    type Reg = RegPaConfig;
    type Value = PaSelectValue;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 7;
}


// Register enum
pub enum PaSelectValue {
        
        /// RFO pin. Output power is limited to +14 dBm.
        RfoPin,
    
        /// PA_BOOST pin. Output power is limited to +20 dBm
        PaBoostPin,
    }

impl RadioRegPartValue for PaSelectValue {
    fn value(&self) -> u8 {
        match self {
           Self::RfoPin => 0b0,
           Self::PaBoostPin => 0b1
        }
    }
}
// Register part
/// Select max output power: Pmax=10.8+0.6*MaxPower[dBm]
pub struct RegPaConfigMaxPower;

impl RadioRegPart for RegPaConfigMaxPower {
    type Reg = RegPaConfig;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 6;
    const END: u8 = 4;
}



// Register part
/// Pout=Pmax-(15-OutputPower) if PaSelect = 0 (RFO pin)
/// Pout=17-(15-OutputPower) if PaSelect = 1 (PA_BOOST pin)
pub struct RegPaConfigOutputPower;

impl RadioRegPart for RegPaConfigOutputPower {
    type Reg = RegPaConfig;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 3;
    const END: u8 = 0;
}



// Register
pub struct RegPaRamp;

impl RadioReg for RegPaRamp {
    const ADDR: u8 = 0x0A;
}
// Register part
/// Rise/Fall time of ramp up/down in FSK
///
/// 0000 --> 3.4 ms
///
/// 0001 --> 2 ms
///
/// 0010 --> 1 ms
///
/// 0011 --> 500 us
///
/// 0100 --> 250 us
///
/// 0101 --> 125 us
///
/// 0110 --> 100 us
///
/// 0111 --> 62 us
///
/// 1000 --> 50 us
///
/// 1001 --> 40 us
///
/// 1010 --> 31 us
///
/// 1011 --> 25 us
///
/// 1100 --> 20 us
///
/// 1101 --> 15 us
///
/// 1110 --> 12 us
///
/// 1111 --> 10 us
pub struct RegPaRampPaRamp;

impl RadioRegPart for RegPaRampPaRamp {
    type Reg = RegPaRamp;
    type Value = PaRampValue;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 3;
    const END: u8 = 0;
}


// Register enum
pub enum PaRampValue {
        
        /// 3.4 ms
        V3_4ms,
    
        /// 2 ms
        V2ms,
    
        /// 1 ms
        V1ms,
    
        /// 500 us
        V500us,
    
        /// 250 us
        V250us,
    
        /// 125 us
        V125us,
    
        /// 100 us
        V100us,
    
        /// 62 us
        V62us,
    
        /// 50 us
        V50us,
    
        /// 40 us
        V40us,
    
        /// 31 us
        V31us,
    
        /// 25 us
        V25us,
    
        /// 20 us
        V20us,
    
        /// 15 us
        V15us,
    
        /// 12 us
        V12us,
    
        /// 10 us
        V10us,
    }

impl RadioRegPartValue for PaRampValue {
    fn value(&self) -> u8 {
        match self {
           Self::V3_4ms => 0b0000,
           Self::V2ms => 0b0001,
           Self::V1ms => 0b0010,
           Self::V500us => 0b0011,
           Self::V250us => 0b0100,
           Self::V125us => 0b0101,
           Self::V100us => 0b0110,
           Self::V62us => 0b0111,
           Self::V50us => 0b1000,
           Self::V40us => 0b1001,
           Self::V31us => 0b1010,
           Self::V25us => 0b1011,
           Self::V20us => 0b1100,
           Self::V15us => 0b1101,
           Self::V12us => 0b1110,
           Self::V10us => 0b1111
        }
    }
}
// Register
pub struct RegOcp;

impl RadioReg for RegOcp {
    const ADDR: u8 = 0x0B;
}
// Register part
/// Enables overload current protection (OCP) for PA:
///
/// 0 --> OCP disabled
///
/// 1 --> OCP enabled
pub struct RegOcpOcpOn;

impl RadioRegPart for RegOcpOcpOn {
    type Reg = RegOcp;
    type Value = OcpOnValue;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 5;
    const END: u8 = 5;
}


// Register enum
pub enum OcpOnValue {
        
        /// OCP disabled
        OcpDisabled,
    
        /// OCP enabled
        OcpEnabled,
    }

impl RadioRegPartValue for OcpOnValue {
    fn value(&self) -> u8 {
        match self {
           Self::OcpDisabled => 0b0,
           Self::OcpEnabled => 0b1
        }
    }
}
// Register part
/// Trimming of OCP current:
/// I max = 45+5*OcpTrim [mA] if OcpTrim &lt;= 15 (120 mA) /
/// I max = -30+10*OcpTrim [mA] if 15 &lt; OcpTrim &lt;= 27 (130 to
/// 240 mA)
/// I max = 240mA for higher settings
/// Default I max = 100mA
pub struct RegOcpOcpTrim;

impl RadioRegPart for RegOcpOcpTrim {
    type Reg = RegOcp;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 4;
    const END: u8 = 0;
}



// Register
pub struct RegLna;

impl RadioReg for RegLna {
    const ADDR: u8 = 0x0C;
}
// Register part
/// LNA gain setting:
///
/// 000 --> not used
///
/// 001 --> G1 = maximum gain
///
/// 010 --> G2
///
/// 011 --> G3
///
/// 100 --> G4
///
/// 101 --> G5
///
/// 110 --> G6 = minimum gain
///
/// 111 --> not used
pub struct RegLnaLnaGain;

impl RadioRegPart for RegLnaLnaGain {
    type Reg = RegLna;
    type Value = LnaGainValue;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 5;
}


// Register enum
pub enum LnaGainValue {
        
        /// not used
        NotUsed,
    
        /// G1 = maximum gain
        G1,
    
        /// G2
        G2,
    
        /// G3
        G3,
    
        /// G4
        G4,
    
        /// G5
        G5,
    
        /// G6 = minimum gain
        G6,
    }

impl RadioRegPartValue for LnaGainValue {
    fn value(&self) -> u8 {
        match self {
           Self::NotUsed => 0b111,
           Self::G1 => 0b001,
           Self::G2 => 0b010,
           Self::G3 => 0b011,
           Self::G4 => 0b100,
           Self::G5 => 0b101,
           Self::G6 => 0b110
        }
    }
}
// Register part
/// High Frequency (RFI_HF) LNA currentadjustment
///
/// 00 --> Default LNA current
///
/// 11 --> Boost on, 150% LNA current
/// registers
pub struct RegLnaLnaBoostHf;

impl RadioRegPart for RegLnaLnaBoostHf {
    type Reg = RegLna;
    type Value = LnaBoostHfValue;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 1;
    const END: u8 = 0;
}


// Register enum
pub enum LnaBoostHfValue {
        
        /// Default LNA current
        DefaultLnaCurrent,
    
        /// Boost on, 150% LNA current
        BoostOn,
    }

impl RadioRegPartValue for LnaBoostHfValue {
    fn value(&self) -> u8 {
        match self {
           Self::DefaultLnaCurrent => 0b00,
           Self::BoostOn => 0b11
        }
    }
}
// Register
pub struct RegFifoAddrPtr;

impl RadioReg for RegFifoAddrPtr {
    const ADDR: u8 = 0x0D;
}
// Register part
/// SPI interface address pointer in FIFO data buffer.
/// 
/// 
pub struct RegFifoAddrPtrFifoAddrPtr;

impl RadioRegPart for RegFifoAddrPtrFifoAddrPtr {
    type Reg = RegFifoAddrPtr;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
}



// Register
pub struct RegFifoTxBaseAddr;

impl RadioReg for RegFifoTxBaseAddr {
    const ADDR: u8 = 0x0E;
}
// Register part
/// write base address in FIFO data buffer for TX modulator
/// 
/// 
pub struct RegFifoTxBaseAddrFifoTxBaseAddr;

impl RadioRegPart for RegFifoTxBaseAddrFifoTxBaseAddr {
    type Reg = RegFifoTxBaseAddr;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
}



// Register
pub struct RegFifoRxBaseAddr;

impl RadioReg for RegFifoRxBaseAddr {
    const ADDR: u8 = 0x0F;
}
// Register part
/// read base address in FIFO data buffer for RX demodulator
/// 
/// 
pub struct RegFifoRxBaseAddrFifoRxBaseAddr;

impl RadioRegPart for RegFifoRxBaseAddrFifoRxBaseAddr {
    type Reg = RegFifoRxBaseAddr;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
}



// Register
pub struct RegFifoRxCurrentAddr;

impl RadioReg for RegFifoRxCurrentAddr {
    const ADDR: u8 = 0x10;
}
// Register part
/// Start address (in data buffer) of last packet received
/// 
/// 
pub struct RegFifoRxCurrentAddrFifoRxCurrentAddr;

impl RadioRegPart for RegFifoRxCurrentAddrFifoRxCurrentAddr {
    type Reg = RegFifoRxCurrentAddr;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
}



// Register
pub struct RegIrqFlagsMask;

impl RadioReg for RegIrqFlagsMask {
    const ADDR: u8 = 0x11;
}
// Register part
/// Timeout interrupt mask: setting this bit masks the corresponding
/// IRQ in RegIrqFlags
pub struct RegIrqFlagsMaskRxTimeoutMask;

impl RadioRegPart for RegIrqFlagsMaskRxTimeoutMask {
    type Reg = RegIrqFlagsMask;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 7;
}



// Register part
/// Packet reception complete interrupt mask: setting this bit masks the
/// corresponding IRQ in RegIrqFlags
pub struct RegIrqFlagsMaskRxDoneMask;

impl RadioRegPart for RegIrqFlagsMaskRxDoneMask {
    type Reg = RegIrqFlagsMask;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 6;
    const END: u8 = 6;
}



// Register part
/// Payload CRC error interrupt mask: setting this bit masks the
/// corresponding IRQ in RegIrqFlags
pub struct RegIrqFlagsMaskPayloadCrcErrorMask;

impl RadioRegPart for RegIrqFlagsMaskPayloadCrcErrorMask {
    type Reg = RegIrqFlagsMask;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 5;
    const END: u8 = 5;
}



// Register part
/// Valid header received in Rx mask: setting this bit masks the
/// corresponding IRQ in RegIrqFlags
pub struct RegIrqFlagsMaskValidHeaderMask;

impl RadioRegPart for RegIrqFlagsMaskValidHeaderMask {
    type Reg = RegIrqFlagsMask;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 4;
    const END: u8 = 4;
}



// Register part
/// FIFO Payload transmission complete interrupt mask: setting this bit
/// masks the corresponding IRQ in RegIrqFlags
pub struct RegIrqFlagsMaskTxDoneMask;

impl RadioRegPart for RegIrqFlagsMaskTxDoneMask {
    type Reg = RegIrqFlagsMask;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 3;
    const END: u8 = 3;
}



// Register part
/// CAD complete interrupt mask: setting this bit masks the
/// corresponding IRQ in RegIrqFlags
pub struct RegIrqFlagsMaskCadDoneMask;

impl RadioRegPart for RegIrqFlagsMaskCadDoneMask {
    type Reg = RegIrqFlagsMask;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 2;
    const END: u8 = 2;
}



// Register part
/// FHSS change channel interrupt mask: setting this bit masks the
/// corresponding IRQ in RegIrqFlags
pub struct RegIrqFlagsMaskFhssChangeChannelMask;

impl RadioRegPart for RegIrqFlagsMaskFhssChangeChannelMask {
    type Reg = RegIrqFlagsMask;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 1;
    const END: u8 = 1;
}



// Register part
/// Cad Detected Interrupt Mask: setting this bit masks the
/// corresponding IRQ in RegIrqFlags
pub struct RegIrqFlagsMaskCadDetectedMask;

impl RadioRegPart for RegIrqFlagsMaskCadDetectedMask {
    type Reg = RegIrqFlagsMask;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 0;
    const END: u8 = 0;
}



// Register
pub struct RegIrqFlags;

impl RadioReg for RegIrqFlags {
    const ADDR: u8 = 0x12;
}
// Register part
/// Timeout interrupt: writing a 1 clears the IRQ
pub struct RegIrqFlagsRxTimeout;

impl RadioRegPart for RegIrqFlagsRxTimeout {
    type Reg = RegIrqFlags;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = true;
    const START: u8 = 7;
    const END: u8 = 7;
}



// Register part
/// Packet reception complete interrupt: writing a 1 clears the IRQ
pub struct RegIrqFlagsRxDone;

impl RadioRegPart for RegIrqFlagsRxDone {
    type Reg = RegIrqFlags;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = true;
    const START: u8 = 6;
    const END: u8 = 6;
}



// Register part
/// Payload CRC error interrupt: writing a 1 clears the IRQ
pub struct RegIrqFlagsPayloadCrcError;

impl RadioRegPart for RegIrqFlagsPayloadCrcError {
    type Reg = RegIrqFlags;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = true;
    const START: u8 = 5;
    const END: u8 = 5;
}



// Register part
/// Valid header received in Rx: writing a 1 clears the IRQ
pub struct RegIrqFlagsValidHeader;

impl RadioRegPart for RegIrqFlagsValidHeader {
    type Reg = RegIrqFlags;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = true;
    const START: u8 = 4;
    const END: u8 = 4;
}



// Register part
/// FIFO Payload transmission complete interrupt: writing a 1 clears
/// the IRQ
pub struct RegIrqFlagsTxDone;

impl RadioRegPart for RegIrqFlagsTxDone {
    type Reg = RegIrqFlags;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = true;
    const START: u8 = 3;
    const END: u8 = 3;
}



// Register part
/// CAD complete: write to clear: writing a 1 clears the IRQ
pub struct RegIrqFlagsCadDone;

impl RadioRegPart for RegIrqFlagsCadDone {
    type Reg = RegIrqFlags;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = true;
    const START: u8 = 2;
    const END: u8 = 2;
}



// Register part
/// FHSS change channel interrupt: writing a 1 clears the IRQ
pub struct RegIrqFlagsFhssChangeChannel;

impl RadioRegPart for RegIrqFlagsFhssChangeChannel {
    type Reg = RegIrqFlags;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = true;
    const START: u8 = 1;
    const END: u8 = 1;
}



// Register part
/// Valid Lora signal detected during CAD operation: writing a 1clears
/// the IRQ
pub struct RegIrqFlagsCadDetected;

impl RadioRegPart for RegIrqFlagsCadDetected {
    type Reg = RegIrqFlags;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = true;
    const START: u8 = 0;
    const END: u8 = 0;
}



// Register
pub struct RegRxNbBytes;

impl RadioReg for RegRxNbBytes {
    const ADDR: u8 = 0x13;
}
// Register part
/// Number of payload bytes of latest packetreceived
/// 
/// 
pub struct RegRxNbBytesFifoRxBytesNb;

impl RadioRegPart for RegRxNbBytesFifoRxBytesNb {
    type Reg = RegRxNbBytes;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
}



// Register
pub struct RegRxHeaderCntValueMsb;

impl RadioReg for RegRxHeaderCntValueMsb {
    const ADDR: u8 = 0x14;
}
// Register part
/// Number of valid headers received since last transition into Rx
/// mode, MSB(15:8). Header and packet counters are reseted in
/// Sleep mode.
/// 
pub struct RegRxHeaderCntValueMsbValidHeaderCntMsb;

impl RadioRegPart for RegRxHeaderCntValueMsbValidHeaderCntMsb {
    type Reg = RegRxHeaderCntValueMsb;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
}



// Register
pub struct RegRxHeaderCntValueLsb;

impl RadioReg for RegRxHeaderCntValueLsb {
    const ADDR: u8 = 0x15;
}
// Register part
/// Number of valid headers received since last transition into Rx
/// mode, LSB(7:0). Header and packet counters are reseted in Sleep
/// mode.
pub struct RegRxHeaderCntValueLsbValidHeaderCntLsb;

impl RadioRegPart for RegRxHeaderCntValueLsbValidHeaderCntLsb {
    type Reg = RegRxHeaderCntValueLsb;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
}



// Register
pub struct RegRxPacketCntValueMsb;

impl RadioReg for RegRxPacketCntValueMsb {
    const ADDR: u8 = 0x16;
}
// Register part
/// Number of valid packets received since last transition into Rx
/// mode, MSB(15:8). Header and packet counters are reseted in
/// Sleep mode.
pub struct RegRxPacketCntValueMsbValidPacketCntMsb;

impl RadioRegPart for RegRxPacketCntValueMsbValidPacketCntMsb {
    type Reg = RegRxPacketCntValueMsb;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = true;
    const START: u8 = 7;
    const END: u8 = 0;
}



// Register
pub struct RegRxPacketCntValueLsb;

impl RadioReg for RegRxPacketCntValueLsb {
    const ADDR: u8 = 0x17;
}
// Register part
/// Number of valid packets received since last transition into Rx
/// mode, LSB(7:0). Header and packet counters are reseted in Sleep
/// mode.
pub struct RegRxPacketCntValueLsbValidPacketCntLsb;

impl RadioRegPart for RegRxPacketCntValueLsbValidPacketCntLsb {
    type Reg = RegRxPacketCntValueLsb;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
}



// Register
pub struct RegModemStat;

impl RadioReg for RegModemStat {
    const ADDR: u8 = 0x18;
}
// Register part
/// Coding rate of last header received
pub struct RegModemStatRxCodingRate;

impl RadioRegPart for RegModemStatRxCodingRate {
    type Reg = RegModemStat;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 5;
}



// Register part
/// Modem clear
pub struct RegModemStatModemClear;

impl RadioRegPart for RegModemStatModemClear {
    type Reg = RegModemStat;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 4;
    const END: u8 = 4;
}



// Register part
/// Header info valid
/// 
pub struct RegModemStatHeaderInfoValid;

impl RadioRegPart for RegModemStatHeaderInfoValid {
    type Reg = RegModemStat;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 3;
    const END: u8 = 3;
}



// Register part
/// RX on-going
pub struct RegModemStatRxOngoing;

impl RadioRegPart for RegModemStatRxOngoing {
    type Reg = RegModemStat;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 2;
    const END: u8 = 2;
}



// Register part
/// Signal synchronized
pub struct RegModemStatSignalSynchronized;

impl RadioRegPart for RegModemStatSignalSynchronized {
    type Reg = RegModemStat;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 1;
    const END: u8 = 1;
}



// Register part
/// Signal detected
pub struct RegModemStatSignalDetected;

impl RadioRegPart for RegModemStatSignalDetected {
    type Reg = RegModemStat;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 0;
    const END: u8 = 0;
}



// Register
pub struct RegPktSnrValue;

impl RadioReg for RegPktSnrValue {
    const ADDR: u8 = 0x19;
}
// Register part
/// Estimation of SNR on last packet received.In two’s compliment
/// format mutiplied by 4.
/// NEEDS FIXING
pub struct RegPktSnrValuePacketSnr;

impl RadioRegPart for RegPktSnrValuePacketSnr {
    type Reg = RegPktSnrValue;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
}



// Register
pub struct RegPktRssiValue;

impl RadioReg for RegPktRssiValue {
    const ADDR: u8 = 0x1A;
}
// Register part
/// RSSI of the latest packet received (dBm):
/// RSSI[dBm] = -157 + Rssi (using HF output port, SNR &gt;= 0)
/// or
/// RSSI[dBm] = -164 + Rssi (using LF output port, SNR &gt;= 0)
/// (see section 5.5.5 for details)
pub struct RegPktRssiValuePacketRssi;

impl RadioRegPart for RegPktRssiValuePacketRssi {
    type Reg = RegPktRssiValue;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
}



// Register
pub struct RegRssiValue;

impl RadioReg for RegRssiValue {
    const ADDR: u8 = 0x1B;
}
// Register part
/// Current RSSI value (dBm)
/// RSSI[dBm] = -157 + Rssi (using HF output port)
/// or
/// RSSI[dBm] = -164 + Rssi (using LF output port)
/// (see section 5.5.5 for details)
pub struct RegRssiValueRssi;

impl RadioRegPart for RegRssiValueRssi {
    type Reg = RegRssiValue;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
}



// Register
pub struct RegHopChannel;

impl RadioReg for RegHopChannel {
    const ADDR: u8 = 0x1C;
}
// Register part
/// PLL failed to lock while attempting a TX/RX/CAD operation
///
/// 1 --> PLL did not lock
///
/// 0 --> PLL did lock
pub struct RegHopChannelPllTimeout;

impl RadioRegPart for RegHopChannelPllTimeout {
    type Reg = RegHopChannel;
    type Value = PllTimeoutValue;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 7;
}


// Register enum
pub enum PllTimeoutValue {
        
        /// PLL did not lock
        PllDidNotLock,
    
        /// PLL did lock
        PllDidLock,
    }

impl RadioRegPartValue for PllTimeoutValue {
    fn value(&self) -> u8 {
        match self {
           Self::PllDidNotLock => 0b1,
           Self::PllDidLock => 0b0
        }
    }
}
// Register part
/// CRC Information extracted from the received packet header
/// (Explicit header mode only)
///
/// 0 --> Header indicates CRC off
///
/// 1 --> Header indicates CRC on
pub struct RegHopChannelCrcOnPayload;

impl RadioRegPart for RegHopChannelCrcOnPayload {
    type Reg = RegHopChannel;
    type Value = CrcOnPayloadValue;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 6;
    const END: u8 = 6;
}


// Register enum
pub enum CrcOnPayloadValue {
        
        /// Header indicates CRC off
        HeaderIndicatesCrcOff,
    
        /// Header indicates CRC on
        HeaderIndicatesCrcOn,
    }

impl RadioRegPartValue for CrcOnPayloadValue {
    fn value(&self) -> u8 {
        match self {
           Self::HeaderIndicatesCrcOff => 0b0,
           Self::HeaderIndicatesCrcOn => 0b1
        }
    }
}
// Register part
/// Current value of frequency hopping channel inuse.
pub struct RegHopChannelFhssPresentChannel;

impl RadioRegPart for RegHopChannelFhssPresentChannel {
    type Reg = RegHopChannel;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 5;
    const END: u8 = 0;
}



// Register
pub struct RegModemConfig1;

impl RadioReg for RegModemConfig1 {
    const ADDR: u8 = 0x1D;
}
// Register part
///
/// 0 --> Explicit Header mode
///
/// 1 --> Implicit Header mode
pub struct RegModemConfig1ImplicitHeaderModeOn;

impl RadioRegPart for RegModemConfig1ImplicitHeaderModeOn {
    type Reg = RegModemConfig1;
    type Value = ImplicitHeaderModeOnValue;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 0;
    const END: u8 = 0;
}


// Register enum
pub enum ImplicitHeaderModeOnValue {
        
        /// Explicit Header mode
        ExplicitHeaderMode,
    
        /// Implicit Header mode
        ImplicitHeaderMode,
    }

impl RadioRegPartValue for ImplicitHeaderModeOnValue {
    fn value(&self) -> u8 {
        match self {
           Self::ExplicitHeaderMode => 0b0,
           Self::ImplicitHeaderMode => 0b1
        }
    }
}
// Register
pub struct RegModemConfig2;

impl RadioReg for RegModemConfig2 {
    const ADDR: u8 = 0x1E;
}
// Register part
///
/// 0 --> normal mode, a single packet is sent
///
/// 1 --> continuous mode, send multiple packets across the FIFO
/// (used for spectral analysis)
pub struct RegModemConfig2TxContinuousMode;

impl RadioRegPart for RegModemConfig2TxContinuousMode {
    type Reg = RegModemConfig2;
    type Value = TxContinuousModeValue;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 3;
    const END: u8 = 3;
}


// Register enum
pub enum TxContinuousModeValue {
        
        /// normal mode, a single packet is sent
        NormalMode,
    
        /// continuous mode, send multiple packets across the FIFO
        ContinuousMode,
    }

impl RadioRegPartValue for TxContinuousModeValue {
    fn value(&self) -> u8 {
        match self {
           Self::NormalMode => 0b0,
           Self::ContinuousMode => 0b1
        }
    }
}
// Register part
/// Enable CRC generation and check on payload:
///
/// 0 --> CRC disable
///
/// 1 --> CRC enable
/// If CRC is needed, RxPayloadCrcOn should beset:
/// - in Implicit header mode: on Tx and Rx side
/// - in Explicit header mode: on the Tx side alone (recovered from the
/// header in Rx side)
pub struct RegModemConfig2RxPayloadCrcOn;

impl RadioRegPart for RegModemConfig2RxPayloadCrcOn {
    type Reg = RegModemConfig2;
    type Value = RxPayloadCrcOnValue;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 2;
    const END: u8 = 2;
}


// Register enum
pub enum RxPayloadCrcOnValue {
        
        /// CRC disable
        CrcDisable,
    
        /// CRC enable
        CrcEnable,
    }

impl RadioRegPartValue for RxPayloadCrcOnValue {
    fn value(&self) -> u8 {
        match self {
           Self::CrcDisable => 0b0,
           Self::CrcEnable => 0b1
        }
    }
}
// Register part
/// RX Time-Out MSB
pub struct RegModemConfig2SymbTimeout;

impl RadioRegPart for RegModemConfig2SymbTimeout {
    type Reg = RegModemConfig2;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 1;
    const END: u8 = 0;
}



// Register
pub struct RegSymbTimeoutLsb;

impl RadioReg for RegSymbTimeoutLsb {
    const ADDR: u8 = 0x1F;
}
// Register part
/// RX Time-Out LSB
/// RX operation time-out value expressed as number of symbols:
/// TimeOut = SymbTimeout  Ts
pub struct RegSymbTimeoutLsbSymbTimeout;

impl RadioRegPart for RegSymbTimeoutLsbSymbTimeout {
    type Reg = RegSymbTimeoutLsb;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
}



// Register
pub struct RegPreambleMsb;

impl RadioReg for RegPreambleMsb {
    const ADDR: u8 = 0x20;
}
// Register part
/// Preamble length MSB, = PreambleLength + 4.25Symbols
/// See 4.1.1 for more details.
/// 
pub struct RegPreambleMsbPreambleLength;

impl RadioRegPart for RegPreambleMsbPreambleLength {
    type Reg = RegPreambleMsb;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
}



// Register
pub struct RegPreambleLsb;

impl RadioReg for RegPreambleLsb {
    const ADDR: u8 = 0x21;
}
// Register part
/// Preamble Length LSB
/// 
/// 
pub struct RegPreambleLsbPreambleLength;

impl RadioRegPart for RegPreambleLsbPreambleLength {
    type Reg = RegPreambleLsb;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
}



// Register
pub struct RegPayloadLength;

impl RadioReg for RegPayloadLength {
    const ADDR: u8 = 0x22;
}
// Register part
/// Payload length in bytes. The register needs to be set in implicit
/// header mode for the expected packet length. A 0 value is not
/// permitted
pub struct RegPayloadLengthPayloadLength;

impl RadioRegPart for RegPayloadLengthPayloadLength {
    type Reg = RegPayloadLength;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
}



// Register
pub struct RegMaxPayloadLength;

impl RadioReg for RegMaxPayloadLength {
    const ADDR: u8 = 0x23;
}
// Register part
/// Maximum payload length; if header payload length exceeds value a
/// header CRC error is generated. Allows filtering of packet with a bad
/// size.
pub struct RegMaxPayloadLengthPayloadMaxLength;

impl RadioRegPart for RegMaxPayloadLengthPayloadMaxLength {
    type Reg = RegMaxPayloadLength;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
}



// Register
pub struct RegHopPeriod;

impl RadioReg for RegHopPeriod {
    const ADDR: u8 = 0x24;
}
// Register part
/// Symbol periods between frequency hops. (0 = disabled). 1st hop
/// always happen after the 1st header symbol
/// 
pub struct RegHopPeriodFreqHoppingPeriod;

impl RadioRegPart for RegHopPeriodFreqHoppingPeriod {
    type Reg = RegHopPeriod;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
}



// Register
pub struct RegFifoRxByteAddr;

impl RadioReg for RegFifoRxByteAddr {
    const ADDR: u8 = 0x25;
}
// Register part
/// Current value of RX databuffer pointer (address of last byte written
/// by Lora receiver)
/// 
pub struct RegFifoRxByteAddrFifoRxByteAddrPtr;

impl RadioRegPart for RegFifoRxByteAddrFifoRxByteAddrPtr {
    type Reg = RegFifoRxByteAddr;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
}



// Register
pub struct RegModemConfig3;

impl RadioReg for RegModemConfig3 {
    const ADDR: u8 = 0x26;
}
// Register part
/// 
pub struct RegModemConfig3Unused;

impl RadioRegPart for RegModemConfig3Unused {
    type Reg = RegModemConfig3;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 4;
}



// Register part
///
/// 0 --> Disabled
///
/// 1 --> Enabled; mandated for when the symbol length exceeds 16ms
pub struct RegModemConfig3LowDataRateOptimize;

impl RadioRegPart for RegModemConfig3LowDataRateOptimize {
    type Reg = RegModemConfig3;
    type Value = LowDataRateOptimizeValue;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 3;
    const END: u8 = 3;
}


// Register enum
pub enum LowDataRateOptimizeValue {
        
        /// Disabled
        Disabled,
    
        /// Enabled; mandated for when the symbol length exceeds 16ms
        Enabled,
    }

impl RadioRegPartValue for LowDataRateOptimizeValue {
    fn value(&self) -> u8 {
        match self {
           Self::Disabled => 0b0,
           Self::Enabled => 0b1
        }
    }
}
// Register part
///
/// 0 --> LNA gain set by register LnaGain
///
/// 1 --> LNA gain set by the internal AGC loop
pub struct RegModemConfig3AgcAutoOn;

impl RadioRegPart for RegModemConfig3AgcAutoOn {
    type Reg = RegModemConfig3;
    type Value = AgcAutoOnValue;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 2;
    const END: u8 = 2;
}


// Register enum
pub enum AgcAutoOnValue {
        
        /// LNA gain set by register LnaGain
        LnaGainSetByRegisterLnaGain,
    
        /// LNA gain set by the internal AGC loop
        LnaGainSetByTheInternalAgcLoop,
    }

impl RadioRegPartValue for AgcAutoOnValue {
    fn value(&self) -> u8 {
        match self {
           Self::LnaGainSetByRegisterLnaGain => 0b0,
           Self::LnaGainSetByTheInternalAgcLoop => 0b1
        }
    }
}
// Register
pub struct Reg0x27;

impl RadioReg for Reg0x27 {
    const ADDR: u8 = 0x27;
}
// Register part
/// Data rate offset value, used in conjunction with AFC
pub struct Reg0x27PpmCorrection;

impl RadioRegPart for Reg0x27PpmCorrection {
    type Reg = Reg0x27;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
}



// Register
pub struct RegFeiMsb;

impl RadioReg for RegFeiMsb {
    const ADDR: u8 = 0x28;
}
// Register part
/// Estimated frequency error from modem
/// MSB of RF Frequency Error
/// F Error = - ---- -- --- --- ---- --- -- ---- ---- --- --2--- -- --   ----- -- --- ---- --- -------
/// F r e q E r r o r   B W  k H z 
/// 2 4
/// F xtal 500
pub struct RegFeiMsbFreqError;

impl RadioRegPart for RegFeiMsbFreqError {
    type Reg = RegFeiMsb;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 3;
    const END: u8 = 0;
}



// Register
pub struct RegFeiMid;

impl RadioReg for RegFeiMid {
    const ADDR: u8 = 0x29;
}
// Register part
/// Middle byte of RF Frequency Error
/// 
pub struct RegFeiMidFreqError;

impl RadioRegPart for RegFeiMidFreqError {
    type Reg = RegFeiMid;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
}



// Register
pub struct RegFeiLsb;

impl RadioReg for RegFeiLsb {
    const ADDR: u8 = 0x2A;
}
// Register part
/// LSB of RF Frequency Error
/// 
pub struct RegFeiLsbFreqError;

impl RadioRegPart for RegFeiLsbFreqError {
    type Reg = RegFeiLsb;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
}



// Register
pub struct RegRssiWideband;

impl RadioReg for RegRssiWideband {
    const ADDR: u8 = 0x2C;
}
// Register part
/// Wideband RSSI measurement used to locally generate a random
/// number
/// 
pub struct RegRssiWidebandRssiWideband;

impl RadioRegPart for RegRssiWidebandRssiWideband {
    type Reg = RegRssiWideband;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
}



// Register
pub struct Reg0x2F;

impl RadioReg for Reg0x2F {
    const ADDR: u8 = 0x2F;
}
// Register part
/// See errata note
pub struct Reg0x2FIfFreq2;

impl RadioRegPart for Reg0x2FIfFreq2 {
    type Reg = Reg0x2F;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
}


