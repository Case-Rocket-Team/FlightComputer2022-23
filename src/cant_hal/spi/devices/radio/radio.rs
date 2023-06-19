use crate::{cant_hal::{spi::spi_interface::{SpiInterface, Selectable, SpiInterfaceDevice}, avionics::SpiDevice}, util::{Writerator, SizedWriterator, SizedWriteratorError}};
use bit_field::BitField;
use embedded_hal::blocking::spi::{Transfer, Write};

use super::{consts::{RadioMode, IRQ, PaConfig, FskDataModulationShaping, FskRampUpRamDown}, radio_layout::RadioReg};
use super::reg_gen::*;
use crate::cant_hal::spi::devices::radio::radio_layout::*;

const FREQUENCY: u32 = 433;

pub enum LoRaError<I: SpiInterface> {
    Uninformative,
    VersionMismatch(u8),
    Select(<I as Selectable>::Error),
    SpiTransfer(<I as Transfer<u8>>::Error),
    SpiWrite(<I as Write<u8>>::Error),
    Transmitting,
    /// Payload was too big so part of it was truncated and never sent
    Truncated
}

pub enum LoRaOrWriteratorError<I: SpiInterface, E> {
    LoRa(LoRaError<I>),
    Writerator(E)
}

pub struct LoRa<I: SpiInterface> {
    interface: I,
    pub explicit_header: bool,
    pub mode: RadioMode,
}

impl<I: SpiInterface> SpiDevice for LoRa<I> {}

impl<I: SpiInterface> SpiInterfaceDevice<I> for LoRa<I> {
    fn new(interface: I) -> Self {
        LoRa {
            interface,
            explicit_header: true,
            mode: RadioMode::Sleep,
        }
    }
}

impl<I: SpiInterface> LoRa<I> {
    pub fn init_radio(&mut self) -> Result<(), LoRaError<I>> {
        self.set_mode(RadioMode::Sleep)?;
        self.set_frequency(FREQUENCY)?;
        self.write_register(RegFifoTxBaseAddr, 0)?;
        self.write_register(RegFifoRxBaseAddr, 0)?;
        let lna = self.read_register(RegLna)?;
        self.write_register(RegLna, lna | 0x03)?;
        //self.write_register(RegCon, 0x04)?;
        self.set_mode(RadioMode::Stdby)?;
        self.set_tx_power(14, 1);
        Ok(())
    }

    pub fn read_version(&mut self) -> Result<u8, LoRaError<I>> {
        self.read_register(RegVersion)
    }

    /*pub fn set_dio0_tx_done(&mut self) -> Result<(), LoRaError<I>> {
        self.write_register(RegDioMapping1, 0b01_00_00_00)
    }*/

    /// Send a payload.
    /// NOTE: Max size is 256 bytes!!!
    pub fn transmit<'a, Iter: Iterator<Item = &'a u8>>(&mut self, payload: Iter) -> Result<(), LoRaError<I>> {
        if self.is_transmitting()? {
            Err(LoRaError::Transmitting)
        } else {
            self.set_mode(RadioMode::Stdby)?;
            if self.explicit_header {
                self.set_explicit_header_mode()?;
            } else {
                self.set_implicit_header_mode()?;
            }

            self.write_register(RegIrqFlags, 0)?;
            self.write_register(RegFifoAddrPtr, 0)?;
            self.write_register(RegPayloadLength, 0)?;

            let mut len = 0;
            let mut truncated = false;
            for byte in payload {
                self.write_register(RegFifo, *byte)?;
                if len == 255 {
                    truncated = true;
                    break;
                }

                len += 1;
            }

            self.write_register(RegPayloadLength, len)?;
            self.set_mode(RadioMode::Tx)?;
            
            if truncated {
                Err(LoRaError::Truncated)
            } else {
                Ok(())
            }
        }
    }

    pub fn read_next_received<T: Writerator<Item = u8>>(&mut self, writerator: &mut T)
            -> Result<(), LoRaOrWriteratorError<I, T::Error>> {
        self.block_until_received_packet().map_err(LoRaOrWriteratorError::LoRa)?;
        self.read_ready_packet(writerator)
    }

    pub fn has_received_packet(&mut self) -> Result<bool, LoRaError<I>> {
        Ok(self.read_register(RegIrqFlags)?.get_bit(6))
    }

    /// Blocks the current thread, returning the size of a packet if one is received or an error is the
    /// task timed out. The timeout can be supplied with None to make it poll indefinitely or
    /// with `Some(timeout_in_mill_seconds)`
    pub fn block_until_received_packet(&mut self) -> Result<(), LoRaError<I>> {
        self.set_mode(RadioMode::RxContinuous)?;
        while !self.has_received_packet()? {}
        Ok(())
    }

    /// Returns the contents of the fifo as a fixed 255 u8 array. This should only be called if there is a
    /// new packet ready to be read.
    pub fn read_ready_packet<T: Writerator<Item = u8>>(&mut self, writerator: &mut T)
            -> Result<(), LoRaOrWriteratorError<I, T::Error>> {
        let mut buffer = [0 as u8; 255];
        self.clear_irq().map_err(LoRaOrWriteratorError::LoRa)?;
        let size = self.get_ready_packet_size().map_err(LoRaOrWriteratorError::LoRa)?;
        let fifo_addr = self.read_register(RegFifoRxCurrentAddr)
                .map_err(LoRaOrWriteratorError::LoRa)?;
        self.write_register(RegFifoAddrPtr, fifo_addr)
                .map_err(LoRaOrWriteratorError::LoRa)?;

        for i in 0..size {
            let byte = self.read_register(RegFifo)
                    .map_err(LoRaOrWriteratorError::LoRa)?;
            writerator.push(byte).map_err(LoRaOrWriteratorError::Writerator)?;
        }
        self.write_register(RegFifoAddrPtr, 0).map_err(LoRaOrWriteratorError::LoRa)?;
        Ok(())
    }

    /// Returns size of a packet read into FIFO. This should only be called if there is a new packet
    /// ready to be read.
    pub fn get_ready_packet_size(&mut self) -> Result<u8, LoRaError<I>> {
        self.read_register(RegRxNbBytes)
    }

    /// Returns true if the radio is currently transmitting a packet.
    pub fn is_transmitting(&mut self) -> Result<bool, LoRaError<I>> {
        let op_mode = self.read_register(RegOpMode)?;
        if (op_mode & RadioMode::Tx.addr()) == RadioMode::Tx.addr()
            || (op_mode & RadioMode::FsTx.addr()) == RadioMode::FsTx.addr() {
            Ok(true)
        } else {
            if (self.read_register(RegIrqFlags)? & IRQ::IrqTxDoneMask.addr()) == 1 {
                self.write_register(RegIrqFlags, IRQ::IrqTxDoneMask.addr())?;
            }
            Ok(false)
        }
    }

    /// Clears the radio's IRQ registers.
    pub fn clear_irq(&mut self) -> Result<(), LoRaError<I>> {
        let irq_flags = self.read_register(RegIrqFlags)?;
        self.write_register(RegIrqFlags, irq_flags)
    }

    /// Sets the transmit power and pin. Levels can range from 0-14 when the output
    /// pin = 0(RFO), and form 0-20 when output pin = 1(PaBoost). Power is in dB.
    /// Default value is `17`.
    pub fn set_tx_power(&mut self, mut level: u32, output_pin: u8,) -> Result<(), LoRaError<I>> {
        if PaConfig::PaOutputRfoPin.addr() == output_pin {
            // RFO
            if level > 14 {
                level = 14;
            }
            self.write_register(RegPaConfig, (0x70 | level) as u8)
        } else {
            // PA BOOST
            if level > 17 {
                if level > 20 {
                    level = 20;
                }
                // subtract 3 from level, so 18 - 20 maps to 15 - 17
                level -= 3;

                // High Power +20 dBm Operation (Semtech SX1276/77/78/79 5.4.3.)
                self.write_register(RegPaDac, 0x87)?;
                self.set_ocp(140)?;
            } else {
                if level < 2 {
                    level = 2;
                }
                //Default value PA_HF/LF or +17dBm
                self.write_register(RegPaDac, 0x84)?;
                self.set_ocp(100)?;
            }
            level -= 2;
            self.write_register(
                RegPaConfig,
                PaConfig::PaBoost.addr() | level as u8,
            )
        }
    }

    /// Sets the over current protection on the radio(mA).
    pub fn set_ocp(&mut self, ma: u8) -> Result<(), LoRaError<I>> {
        let mut ocp_trim: u8 = 27;

        if ma <= 120 {
            ocp_trim = (ma - 45) / 5;
        } else if ma <= 240 {
            ocp_trim = (ma + 30) / 10;
        }
        self.write_register(RegOcp, 0x20 | (0x1F & ocp_trim))
    }

    /// Sets the state of the radio. Default mode after initiation is `Standby`.
    pub fn set_mode(&mut self, mode: RadioMode) -> Result<(), LoRaError<I>> {
        if self.explicit_header {
            self.set_explicit_header_mode()?;
        } else {
            self.set_implicit_header_mode()?;
        }
        self.write_register(
            RegOpMode,
            RadioMode::LongRangeMode.addr() | mode.addr(),
        )?;

        self.mode = mode;
        Ok(())
    }

    /// Sets the frequency of the radio. Values are in megahertz.
    /// I.E. 915 MHz must be used for North America. Check regulation for your area.
    pub fn set_frequency(&mut self, freq: u32) -> Result<(), LoRaError<I>> {
        // calculate register values
        let base = 1;
        let frf = (freq * (base << 19)) / 32;
        // write registers
        self.write_register(
            RegFrfMsb,
            ((frf & 0x00FF_0000) >> 16) as u8,
        )?;
        self.write_register(RegFrfMid, ((frf & 0x0000_FF00) >> 8) as u8)?;
        self.write_register(RegFrfLsb, (frf & 0x0000_00FF) as u8)
    }

    /// Sets the radio to use an explicit header. Default state is `ON`.
    fn set_explicit_header_mode(&mut self) -> Result<(), LoRaError<I>> {
        let reg_modem_config_1 = self.read_register(RegModemConfig1)?;
        self.write_register(RegModemConfig1, reg_modem_config_1 & 0xfe)?;
        self.explicit_header = true;
        Ok(())
    }

    /// Sets the radio to use an implicit header. Default state is `OFF`.
    fn set_implicit_header_mode(&mut self) -> Result<(), LoRaError<I>> {
        let reg_modem_config_1 = self.read_register(RegModemConfig1)?;
        self.write_register(RegModemConfig1, reg_modem_config_1 & 0x01)?;
        self.explicit_header = false;
        Ok(())
    }

    /// Sets the spreading factor of the radio. Supported values are between 6 and 12.
    /// If a spreading factor of 6 is set, implicit header mode must be used to transmit
    /// and receive packets. Default value is `7`.
    pub fn set_spreading_factor(
        &mut self,
        mut sf: u8,
    ) -> Result<(), LoRaError<I>> {
        if sf < 6 {
            sf = 6;
        } else if sf > 12 {
            sf = 12;
        }

        if sf == 6 {
            self.write_register(RegDetectOptimize, 0xc5)?;
            self.write_register(RegDetectionThreshold, 0x0c)?;
        } else {
            self.write_register(RegDetectOptimize, 0xc3)?;
            self.write_register(RegDetectionThreshold, 0x0a)?;
        }
        let modem_config_2 = self.read_register(RegModemConfig2)?;
        self.write_register(
            RegModemConfig2,
            (modem_config_2 & 0x0f) | ((sf << 4) & 0xf0),
        )?;
        self.set_ldo_flag()?;
        Ok(())
    }

    /// Sets the signal bandwidth of the radio. Supported values are: `7800 Hz`, `10400 Hz`,
    /// `15600 Hz`, `20800 Hz`, `31250 Hz`,`41700 Hz` ,`62500 Hz`,`125000 Hz` and `250000 Hz`
    /// Default value is `125000 Hz`
    /// See p. 4 of SX1276_77_8_ErrataNote_1.1_STD.pdf for Errata implemetation
    pub fn set_signal_bandwidth(
        &mut self,
        sbw: i64,
    ) -> Result<(), LoRaError<I>> {
        let bw: i64 = match sbw {
            7_800 => 0,
            10_400 => 1,
            15_600 => 2,
            20_800 => 3,
            31_250 => 4,
            41_700 => 5,
            62_500 => 6,
            125_000 => 7,
            250_000 => 8,
            _ => 9,
        };

        if bw == 9 {
            self.write_register(RegHighBWOptimize1, 0x02)?;
            self.write_register(RegHighBWOptimize2, 0x64)?;
        } else {
            self.write_register(RegHighBWOptimize1, 0x03)?;
            self.write_register(RegHighBWOptimize2, 0x65)?;
        }

        let modem_config_1 = self.read_register(RegModemConfig1)?;
        self.write_register(
            RegModemConfig1,
            (modem_config_1 & 0x0f) | ((bw << 4) as u8),
        )?;
        self.set_ldo_flag()?;
        Ok(())
    }

    /// Sets the coding rate of the radio with the numerator fixed at 4. Supported values
    /// are between `5` and `8`, these correspond to coding rates of `4/5` and `4/8`.
    /// Default value is `5`.
    pub fn set_coding_rate_4(
        &mut self,
        mut denominator: u8,
    ) -> Result<(), LoRaError<I>> {
        if denominator < 5 {
            denominator = 5;
        } else if denominator > 8 {
            denominator = 8;
        }
        let cr = denominator - 4;
        let modem_config_1 = self.read_register(RegModemConfig1)?;
        self.write_register(
            RegModemConfig1,
            (modem_config_1 & 0xf1) | (cr << 1),
        )
    }

    /// Sets the preamble length of the radio. Values are between 6 and 65535.
    /// Default value is `8`.
    pub fn set_preamble_length(
        &mut self,
        length: i64,
    ) -> Result<(), LoRaError<I>> {
        self.write_register(RegPreambleMsb, (length >> 8) as u8)?;
        self.write_register(RegPreambleLsb, length as u8)
    }

    /// Enables are disables the radio's CRC check. Default value is `false`.
    pub fn set_crc(&mut self, value: bool) -> Result<(), LoRaError<I>> {
        let modem_config_2 = self.read_register(RegModemConfig2)?;
        if value {
            self.write_register(RegModemConfig2, modem_config_2 | 0x04)
        } else {
            self.write_register(RegModemConfig2, modem_config_2 & 0xfb)
        }
    }

    /// Inverts the radio's IQ signals. Default value is `false`.
    pub fn set_invert_iq(&mut self, value: bool) -> Result<(), LoRaError<I>> {
        if value {
            self.write_register(RegInvertIQ, 0x66)?;
            self.write_register(RegInvertIQ2, 0x19)
        } else {
            self.write_register(RegInvertIQ, 0x27)?;
            self.write_register(RegInvertIQ2, 0x1d)
        }
    }

    /// Returns the spreading factor of the radio.
    pub fn get_spreading_factor(&mut self) -> Result<u8, LoRaError<I>> {
        Ok(self.read_register(RegModemConfig2)? >> 4)
    }

    /// Returns the signal bandwidth of the radio.
    pub fn get_signal_bandwidth(&mut self) -> Result<i64, LoRaError<I>> {
        let bw = self.read_register(RegModemConfig1)? >> 4;
        let bw = match bw {
            0 => 7_800,
            1 => 10_400,
            2 => 15_600,
            3 => 20_800,
            4 => 31_250,
            5 => 41_700,
            6 => 62_500,
            7 => 125_000,
            8 => 250_000,
            9 => 500_000,
            _ => -1,
        };
        Ok(bw)
    }

    /// Returns the RSSI of the last received packet.
    pub fn get_packet_rssi(&mut self) -> Result<i32, LoRaError<I>> {
        Ok(i32::from(self.read_register(RegPktRssiValue)?) - 157)
    }

    /// Returns the signal to noise radio of the the last received packet.
    pub fn get_packet_snr(&mut self) -> Result<f64, LoRaError<I>> {
        Ok(f64::from(
            self.read_register(RegPktSnrValue)?,
        ))
    }

/*
    /// Returns the frequency error of the last received packet in Hz.
    pub fn get_packet_frequency_error(&mut self) -> Result<i64, LoRaError<I>> {
        let mut freq_error: i32 = 0;
        freq_error = i32::from(self.read_register(RegFreqErrorMsb)? & 0x7);
        freq_error <<= 8i64;
        freq_error += i32::from(self.read_register(RegFreqErrorMid)?);
        freq_error <<= 8i64;
        freq_error += i32::from(self.read_register(RegFreqErrorLsb)?);

        let f_xtal = 32_000_000; // FXOSC: crystal oscillator (XTAL) frequency (2.5. Chip Specification, p. 14)
        let f_error = ((f64::from(freq_error) * (1i64 << 24) as f64) / f64::from(f_xtal))
            * (self.get_signal_bandwidth()? as f64 / 500_000.0f64); // p. 37
        Ok(f_error as i64)
    }*/

    fn set_ldo_flag(&mut self) -> Result<(), LoRaError<I>> {
        let sw = self.get_signal_bandwidth()?;
        // Section 4.1.1.5
        let symbol_duration = 1000 / (sw / ((1 as i64) << self.get_spreading_factor()?));

        // Section 4.1.1.6
        let ldo_on = symbol_duration > 16;

        let mut config_3 = self.read_register(RegModemConfig3)?;
        config_3.set_bit(3, ldo_on);
        self.write_register(RegModemConfig3, config_3)
    }

    fn read_register<R: RadioReg>(&mut self, reg: R) -> Result<u8, LoRaError<I>> {
        self.interface.select().map_err(LoRaError::Select)?;
        let mut buffer = [R::ADDR & 0x7f, 0];
        log::info!("Read reg: {:x}", buffer[0]);
        let transfer = self.interface.transfer(&mut buffer).map_err(LoRaError::SpiTransfer)?;
        self.interface.deselect().map_err(LoRaError::Select)?;
        Ok(transfer[1])
    }

    fn write_register<R: RadioReg>(&mut self, reg: R, byte: u8) -> Result<(), LoRaError<I>> {
        self.interface.select().map_err(LoRaError::Select)?;
        let buffer = [R::ADDR | 0x80, byte];
        self.interface.write(&buffer).map_err(LoRaError::SpiWrite)?;
        self.interface.deselect().map_err(LoRaError::Select)?;
        Ok(())
    }

    pub fn put_in_fsk_mode(&mut self) -> Result<(), LoRaError<I>> {
        // Put in FSK mode
        let mut op_mode: u8 = 0x0;
        op_mode
            .set_bit(7, false) // FSK mode
            .set_bits(5..6, 0x00) // FSK modulation
            .set_bit(3, false) //Low freq registers
            .set_bits(0..2, 0b011); // Mode

        self.write_register(RegOpMode, op_mode)
    }

    pub fn set_fsk_pa_ramp(
        &mut self,
        modulation_shaping: FskDataModulationShaping,
        ramp: FskRampUpRamDown,
    ) -> Result<(), LoRaError<I>> {
        let mut pa_ramp: u8 = 0x0;
        pa_ramp
            .set_bits(5..6, modulation_shaping as u8)
            .set_bits(0..3, ramp as u8);

        self.write_register(RegPaRamp, pa_ramp)
    }
}