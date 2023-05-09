
// GENERATED FILE, DO NOT EDIT!

use crate::cant_hal::spi::devices::radio::radio_layout::*; 

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegFifo --
pub struct RegFifo;

impl RadioReg for RegFifo {
    const ADDR: u8 = 0x00;
}
// GENERATED FILE, DO NOT EDIT!
// Register part

impl RadioRegPart for RegFifo {
    type Reg = RegFifo;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart Fifo --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegOpMode --
pub struct RegOpMode;

impl RadioReg for RegOpMode {
    const ADDR: u8 = 0x01;
}
// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b10000000;
}

// GENERATED FILE, DO NOT EDIT!
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
// -- End RegPart LongRangeMode --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b1000000;
}

// GENERATED FILE, DO NOT EDIT!
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
// -- End RegPart AccessSharedReg --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b1000;
}

// GENERATED FILE, DO NOT EDIT!
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
// -- End RegPart LowFrequencyModeOn --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b111;
}

// GENERATED FILE, DO NOT EDIT!
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
// -- End RegPart Mode --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegFrfMsb --
pub struct RegFrfMsb;

impl RadioReg for RegFrfMsb {
    const ADDR: u8 = 0x06;
}
// GENERATED FILE, DO NOT EDIT!
// Register part

/// MSB of RF carrier frequency
pub struct RegFrfMsbFrf;

impl RadioRegPart for RegFrfMsbFrf {
    type Reg = RegFrfMsb;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart Frf --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegFrfMid --
pub struct RegFrfMid;

impl RadioReg for RegFrfMid {
    const ADDR: u8 = 0x07;
}
// GENERATED FILE, DO NOT EDIT!
// Register part

/// MSB of RF carrier frequency
/// 
pub struct RegFrfMidFrf;

impl RadioRegPart for RegFrfMidFrf {
    type Reg = RegFrfMid;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart Frf --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegFrfLsb --
pub struct RegFrfLsb;

impl RadioReg for RegFrfLsb {
    const ADDR: u8 = 0x08;
}
// GENERATED FILE, DO NOT EDIT!
// Register part

/// NEEDS FIX LSB of RF carrier frequency
/// Resolution is 61.035 Hz if F(XOSC) = 32 MHz. Default value is
/// 0x6c8000 = 434 MHz. Register values must be modified only when
/// device is in SLEEP or STAND-BY mode.
/// for RF blocks
pub struct RegFrfLsbFrf;

impl RadioRegPart for RegFrfLsbFrf {
    type Reg = RegFrfLsb;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = true;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart Frf --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegPaConfig --
pub struct RegPaConfig;

impl RadioReg for RegPaConfig {
    const ADDR: u8 = 0x09;
}
// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b10000000;
}

// GENERATED FILE, DO NOT EDIT!
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
// -- End RegPart PaSelect --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b1110000;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart MaxPower --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b1111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart OutputPower --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegPaRamp --
pub struct RegPaRamp;

impl RadioReg for RegPaRamp {
    const ADDR: u8 = 0x0A;
}
// GENERATED FILE, DO NOT EDIT!
// Register part

impl RadioRegPart for RegPaRamp {
    type Reg = RegPaRamp;
    type Value = PaRampValue;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 3;
    const END: u8 = 0;
    const PART_MASK: u8 = 0b1111;
}

// GENERATED FILE, DO NOT EDIT!
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
// -- End RegPart PaRamp --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegOcp --
pub struct RegOcp;

impl RadioReg for RegOcp {
    const ADDR: u8 = 0x0B;
}
// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b100000;
}

// GENERATED FILE, DO NOT EDIT!
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
// -- End RegPart OcpOn --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b11111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart OcpTrim --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegLna --
pub struct RegLna;

impl RadioReg for RegLna {
    const ADDR: u8 = 0x0C;
}
// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b11100000;
}

// GENERATED FILE, DO NOT EDIT!
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
// -- End RegPart LnaGain --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b11;
}

// GENERATED FILE, DO NOT EDIT!
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
// -- End RegPart LnaBoostHf --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegFifoAddrPtr --
pub struct RegFifoAddrPtr;

impl RadioReg for RegFifoAddrPtr {
    const ADDR: u8 = 0x0D;
}
// GENERATED FILE, DO NOT EDIT!
// Register part

impl RadioRegPart for RegFifoAddrPtr {
    type Reg = RegFifoAddrPtr;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart FifoAddrPtr --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegFifoTxBaseAddr --
pub struct RegFifoTxBaseAddr;

impl RadioReg for RegFifoTxBaseAddr {
    const ADDR: u8 = 0x0E;
}
// GENERATED FILE, DO NOT EDIT!
// Register part

impl RadioRegPart for RegFifoTxBaseAddr {
    type Reg = RegFifoTxBaseAddr;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart FifoTxBaseAddr --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegFifoRxBaseAddr --
pub struct RegFifoRxBaseAddr;

impl RadioReg for RegFifoRxBaseAddr {
    const ADDR: u8 = 0x0F;
}
// GENERATED FILE, DO NOT EDIT!
// Register part

impl RadioRegPart for RegFifoRxBaseAddr {
    type Reg = RegFifoRxBaseAddr;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart FifoRxBaseAddr --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegFifoRxCurrentAddr --
pub struct RegFifoRxCurrentAddr;

impl RadioReg for RegFifoRxCurrentAddr {
    const ADDR: u8 = 0x10;
}
// GENERATED FILE, DO NOT EDIT!
// Register part

impl RadioRegPart for RegFifoRxCurrentAddr {
    type Reg = RegFifoRxCurrentAddr;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart FifoRxCurrentAddr --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegIrqFlagsMask --
pub struct RegIrqFlagsMask;

impl RadioReg for RegIrqFlagsMask {
    const ADDR: u8 = 0x11;
}
// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b10000000;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart RxTimeoutMask --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b1000000;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart RxDoneMask --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b100000;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart PayloadCrcErrorMask --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b10000;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart ValidHeaderMask --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b1000;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart TxDoneMask --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b100;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart CadDoneMask --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b10;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart FhssChangeChannelMask --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b1;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart CadDetectedMask --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegIrqFlags --
pub struct RegIrqFlags;

impl RadioReg for RegIrqFlags {
    const ADDR: u8 = 0x12;
}
// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b10000000;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart RxTimeout --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b1000000;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart RxDone --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b100000;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart PayloadCrcError --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b10000;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart ValidHeader --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b1000;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart TxDone --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b100;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart CadDone --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b10;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart FhssChangeChannel --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b1;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart CadDetected --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegRxNbBytes --
pub struct RegRxNbBytes;

impl RadioReg for RegRxNbBytes {
    const ADDR: u8 = 0x13;
}
// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart FifoRxBytesNb --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegRxHeaderCntValueMsb --
pub struct RegRxHeaderCntValueMsb;

impl RadioReg for RegRxHeaderCntValueMsb {
    const ADDR: u8 = 0x14;
}
// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart ValidHeaderCntMsb --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegRxHeaderCntValueLsb --
pub struct RegRxHeaderCntValueLsb;

impl RadioReg for RegRxHeaderCntValueLsb {
    const ADDR: u8 = 0x15;
}
// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart ValidHeaderCntLsb --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegRxPacketCntValueMsb --
pub struct RegRxPacketCntValueMsb;

impl RadioReg for RegRxPacketCntValueMsb {
    const ADDR: u8 = 0x16;
}
// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart ValidPacketCntMsb --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegRxPacketCntValueLsb --
pub struct RegRxPacketCntValueLsb;

impl RadioReg for RegRxPacketCntValueLsb {
    const ADDR: u8 = 0x17;
}
// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart ValidPacketCntLsb --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegModemStat --
pub struct RegModemStat;

impl RadioReg for RegModemStat {
    const ADDR: u8 = 0x18;
}
// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b11100000;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart RxCodingRate --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b10000;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart ModemClear --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b1000;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart HeaderInfoValid --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b100;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart RxOngoing --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b10;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart SignalSynchronized --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b1;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart SignalDetected --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegPktSnrValue --
pub struct RegPktSnrValue;

impl RadioReg for RegPktSnrValue {
    const ADDR: u8 = 0x19;
}
// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart PacketSnr --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegPktRssiValue --
pub struct RegPktRssiValue;

impl RadioReg for RegPktRssiValue {
    const ADDR: u8 = 0x1A;
}
// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart PacketRssi --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegRssiValue --
pub struct RegRssiValue;

impl RadioReg for RegRssiValue {
    const ADDR: u8 = 0x1B;
}
// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart Rssi --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegHopChannel --
pub struct RegHopChannel;

impl RadioReg for RegHopChannel {
    const ADDR: u8 = 0x1C;
}
// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b10000000;
}

// GENERATED FILE, DO NOT EDIT!
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
// -- End RegPart PllTimeout --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b1000000;
}

// GENERATED FILE, DO NOT EDIT!
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
// -- End RegPart CrcOnPayload --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart FhssPresentChannel --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegModemConfig1 --
pub struct RegModemConfig1;

impl RadioReg for RegModemConfig1 {
    const ADDR: u8 = 0x1D;
}
// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b1;
}

// GENERATED FILE, DO NOT EDIT!
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
// -- End RegPart ImplicitHeaderModeOn --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegModemConfig2 --
pub struct RegModemConfig2;

impl RadioReg for RegModemConfig2 {
    const ADDR: u8 = 0x1E;
}
// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b1000;
}

// GENERATED FILE, DO NOT EDIT!
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
// -- End RegPart TxContinuousMode --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b100;
}

// GENERATED FILE, DO NOT EDIT!
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
// -- End RegPart RxPayloadCrcOn --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b11;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart SymbTimeout --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegSymbTimeoutLsb --
pub struct RegSymbTimeoutLsb;

impl RadioReg for RegSymbTimeoutLsb {
    const ADDR: u8 = 0x1F;
}
// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart SymbTimeout --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegPreambleMsb --
pub struct RegPreambleMsb;

impl RadioReg for RegPreambleMsb {
    const ADDR: u8 = 0x20;
}
// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart PreambleLength --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegPreambleLsb --
pub struct RegPreambleLsb;

impl RadioReg for RegPreambleLsb {
    const ADDR: u8 = 0x21;
}
// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart PreambleLength --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegPayloadLength --
pub struct RegPayloadLength;

impl RadioReg for RegPayloadLength {
    const ADDR: u8 = 0x22;
}
// GENERATED FILE, DO NOT EDIT!
// Register part

impl RadioRegPart for RegPayloadLength {
    type Reg = RegPayloadLength;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart PayloadLength --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegMaxPayloadLength --
pub struct RegMaxPayloadLength;

impl RadioReg for RegMaxPayloadLength {
    const ADDR: u8 = 0x23;
}
// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart PayloadMaxLength --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegHopPeriod --
pub struct RegHopPeriod;

impl RadioReg for RegHopPeriod {
    const ADDR: u8 = 0x24;
}
// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart FreqHoppingPeriod --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegFifoRxByteAddr --
pub struct RegFifoRxByteAddr;

impl RadioReg for RegFifoRxByteAddr {
    const ADDR: u8 = 0x25;
}
// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart FifoRxByteAddrPtr --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegModemConfig3 --
pub struct RegModemConfig3;

impl RadioReg for RegModemConfig3 {
    const ADDR: u8 = 0x26;
}
// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b11110000;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart Unused --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b1000;
}

// GENERATED FILE, DO NOT EDIT!
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
// -- End RegPart LowDataRateOptimize --

// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b100;
}

// GENERATED FILE, DO NOT EDIT!
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
// -- End RegPart AgcAutoOn --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register Reg0x27 --
pub struct Reg0x27;

impl RadioReg for Reg0x27 {
    const ADDR: u8 = 0x27;
}
// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart PpmCorrection --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegFeiMsb --
pub struct RegFeiMsb;

impl RadioReg for RegFeiMsb {
    const ADDR: u8 = 0x28;
}
// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b1111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart FreqError --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegFeiMid --
pub struct RegFeiMid;

impl RadioReg for RegFeiMid {
    const ADDR: u8 = 0x29;
}
// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart FreqError --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegFeiLsb --
pub struct RegFeiLsb;

impl RadioReg for RegFeiLsb {
    const ADDR: u8 = 0x2A;
}
// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart FreqError --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegRssiWideband --
pub struct RegRssiWideband;

impl RadioReg for RegRssiWideband {
    const ADDR: u8 = 0x2C;
}
// GENERATED FILE, DO NOT EDIT!
// Register part

impl RadioRegPart for RegRssiWideband {
    type Reg = RegRssiWideband;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart RssiWideband --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register Reg0x2F --
pub struct Reg0x2F;

impl RadioReg for Reg0x2F {
    const ADDR: u8 = 0x2F;
}
// GENERATED FILE, DO NOT EDIT!
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
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart IfFreq2 --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegDetectOptimize --
pub struct RegDetectOptimize;

impl RadioReg for RegDetectOptimize {
    const ADDR: u8 = 0x31;
}
// GENERATED FILE, DO NOT EDIT!
// Register part

/// LoRa Detection Optimize
///
/// 0x03 --> SF7 to SF12
///
/// 0x05 --> SF6
pub struct RegDetectOptimizeDetectionOptimize;

impl RadioRegPart for RegDetectOptimizeDetectionOptimize {
    type Reg = RegDetectOptimize;
    type Value = DetectionOptimizeValue;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 2;
    const END: u8 = 0;
    const PART_MASK: u8 = 0b111;
}

// GENERATED FILE, DO NOT EDIT!
// Register enum
pub enum DetectionOptimizeValue {
        
        /// SF7 to SF12
        Sf7ToSf12,
    
        /// SF6
        Sf6,
    }

impl RadioRegPartValue for DetectionOptimizeValue {
    fn value(&self) -> u8 {
        match self {
           Self::Sf7ToSf12 => 0x03,
           Self::Sf6 => 0x05
        }
    }
}
// -- End RegPart DetectionOptimize --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegInvertIQ --
pub struct RegInvertIQ;

impl RadioReg for RegInvertIQ {
    const ADDR: u8 = 0x33;
}
// GENERATED FILE, DO NOT EDIT!
// Register part

impl RadioRegPart for RegInvertIQ {
    type Reg = RegInvertIQ;
    type Value = InvertIQValue;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 6;
    const END: u8 = 6;
    const PART_MASK: u8 = 0b1000000;
}

// GENERATED FILE, DO NOT EDIT!
// Register enum
pub enum InvertIQValue {
        
        /// normal mode
        NormalMode,
    
        /// I and Q signals are inverted
        IAndQSignalsAreInverted,
    }

impl RadioRegPartValue for InvertIQValue {
    fn value(&self) -> u8 {
        match self {
           Self::NormalMode => 0b0,
           Self::IAndQSignalsAreInverted => 0b1
        }
    }
}
// -- End RegPart InvertIQ --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegHighBWOptimize1 --
pub struct RegHighBWOptimize1;

impl RadioReg for RegHighBWOptimize1 {
    const ADDR: u8 = 0x36;
}
// GENERATED FILE, DO NOT EDIT!
// Register part

impl RadioRegPart for RegHighBWOptimize1 {
    type Reg = RegHighBWOptimize1;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart HighBWOptimize1 --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegDetectionThreshold --
pub struct RegDetectionThreshold;

impl RadioReg for RegDetectionThreshold {
    const ADDR: u8 = 0x37;
}
// GENERATED FILE, DO NOT EDIT!
// Register part

impl RadioRegPart for RegDetectionThreshold {
    type Reg = RegDetectionThreshold;
    type Value = DetectionThresholdValue;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!
// Register enum
pub enum DetectionThresholdValue {
        
        /// SF7 to SF12
        Sf7ToSf12,
    
        /// SF6
        Sf6,
    }

impl RadioRegPartValue for DetectionThresholdValue {
    fn value(&self) -> u8 {
        match self {
           Self::Sf7ToSf12 => 0x0A,
           Self::Sf6 => 0x0C
        }
    }
}
// -- End RegPart DetectionThreshold --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegSyncWord --
pub struct RegSyncWord;

impl RadioReg for RegSyncWord {
    const ADDR: u8 = 0x39;
}
// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegHighBWOptimize2 --
pub struct RegHighBWOptimize2;

impl RadioReg for RegHighBWOptimize2 {
    const ADDR: u8 = 0x3A;
}
// GENERATED FILE, DO NOT EDIT!
// Register part

impl RadioRegPart for RegHighBWOptimize2 {
    type Reg = RegHighBWOptimize2;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart HighBWOptimize2 --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegInvertIQ2 --
pub struct RegInvertIQ2;

impl RadioReg for RegInvertIQ2 {
    const ADDR: u8 = 0x3B;
}
// GENERATED FILE, DO NOT EDIT!
// Register part

impl RadioRegPart for RegInvertIQ2 {
    type Reg = RegInvertIQ2;
    type Value = u8;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart InvertIQ2 --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegVersion --
pub struct RegVersion;

impl RadioReg for RegVersion {
    const ADDR: u8 = 0x42;
}
// GENERATED FILE, DO NOT EDIT!
// Register part

impl RadioRegPart for RegVersion {
    type Reg = RegVersion;
    type Value = u8;
    const WRITABLE: bool = false;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 7;
    const END: u8 = 0;
    const PART_MASK: u8 = 0b11111111;
}

// GENERATED FILE, DO NOT EDIT!

// -- End RegPart Version --

// GENERATED FILE, DO NOT EDIT!
// -- Begin Register RegPaDac --
pub struct RegPaDac;

impl RadioReg for RegPaDac {
    const ADDR: u8 = 0x4d;
}
// GENERATED FILE, DO NOT EDIT!
// Register part

impl RadioRegPart for RegPaDac {
    type Reg = RegPaDac;
    type Value = PaDacValue;
    const WRITABLE: bool = true;
    const READABLE: bool = true;
    const TRIGGERABLE: bool = false;
    const CLEARABLE: bool = false;
    const START: u8 = 2;
    const END: u8 = 0;
    const PART_MASK: u8 = 0b111;
}

// GENERATED FILE, DO NOT EDIT!
// Register enum
pub enum PaDacValue {
        
        /// Default value
        DefaultValue,
    
        /// +20dBm on PA_BOOST when OutputPower=1111
        _20dBmOnPaBoostWhenOutputPower,
    }

impl RadioRegPartValue for PaDacValue {
    fn value(&self) -> u8 {
        match self {
           Self::DefaultValue => 0x04,
           Self::_20dBmOnPaBoostWhenOutputPower => 0x07
        }
    }
}
// -- End RegPart PaDac --
