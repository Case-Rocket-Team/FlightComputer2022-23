use crate::radio_registers;

radio_registers! {
    RadioReg;
    0x00,RegFifo,RegFifo,0x00,0x00,"FIFO read/write access","FIFO read/write access"
    0x01,RegOpMode,RegOpMode,0x01,0x01,"Operating mode & LoRa / FSK selection","Operating mode & LoRa / FSK selection"
    0x02,RegBitrateMsb,UNUSED,0x1A,0x1A,"Bit Rate setting, Most Significant Bits","Bit Rate setting, Most Significant Bits"
    0x03,RegBitrateLsb,UNUSED,0x0B,0x0B,"Bit Rate setting, Least Significant Bits","Bit Rate setting, Least Significant Bits"
    0x04,RegFdevMsb,UNUSED,0x00,0x00,"Frequency Deviation setting, Most Significant Bits","Frequency Deviation setting, Most Significant Bits"
    0x05,RegFdevLsb,UNUSED,0x52,0x52,"Frequency Deviation setting, Least Significant Bits","Frequency Deviation setting, Least Significant Bits"
    0x06,RegFrfMsb,RegFrfMsb,0xE4,0xE4,"RF Carrier Frequency, Most Significant Bits","RF Carrier Frequency, Most Significant Bits"
    0x07,RegFrfMid,RegFrfMid,0xC0,0xC0,"RF Carrier Frequency, Intermediate Bits","RF Carrier Frequency, Intermediate Bits"
    0x08,RegFrfLsb,RegFrfLsb,0x00,0x00,"RF Carrier Frequency, Least Significant Bits","RF Carrier Frequency, Least Significant Bits"
    0x09,RegPaConfig,RegPaConfig,0x0F,0x0F,"PA selection and Output Power control","PA selection and Output Power control"
    0x0A,RegPaRamp,RegPaRamp,0x19,0x19,"Control of PA ramp time, low phase PLL","Control of PA ramp time, low phase PLL"
    0x0B,RegOcp,RegOcp,0x2B,0x2B,"Over Current Protection control","Over Current Protection control"
    0x0C,RegLna,RegLna,0x20,0x20,"LNA settings","LNA settings"
    0x0D,RegRxConfig,RegFifoAddrPtr,0x08,0x0E,"AFC, AGC, ctrl","FIFO SPI pointer"
    0x0E,RegRssiConfig,RegFifoTxBaseAddr,0x02,0x02,"RSSI","Start Tx data"
    0x0F,RegRssiCollision,RegFifoRxBaseAddr,0x0A,0x0A,"RSSI Collision detector","Start Rx data"
    0x10,RegRssiThresh,RegIrqFlags,0xFF,0xFF,"RSSI Threshold control","LoRa state flags"
    0x11,RegRssiValue,RegIrqFlagsMask,None,None,"RSSI value in dBm","Optional flag mask"
    0x12,RegRxBw,RegFreqIfMsb,0x15,0x15,"Channel Filter BW Control","IF Frequency"
    0x13,RegAfcBw,RegFreqIFLsb,0x0B,0x0B,"AFC Channel Filter BW","AFC Channel Filter BW"
    0x14,RegOokPeak,RegSymbTimeoutMsb,0x28,0x28,"OOK demodulator","Receiver timeout value"
    0x15,RegOokFix,RegSymbTimeoutLsb,0x0C,0x0C,"Threshold of the OOK demod","Threshold of the OOK demod"
    0x16,RegOokAvg,RegTxCfg,0x12,0x12,"Average of the OOK demod","LoRa transmit"
    0x17,Reserved17,RegPayloadLength,0x47,0x47,"-","LoRa parameters"
    0x18,Reserved18,RegPreambleMsb,0x32,0x32,"-","Size of preamble"
    0x19,Reserved19,RegPreambleLsb,0x3E,0x3E,"-","Size of preamble"
    0x1A,RegAfcFei,RegModulationCfg,0x00,0x00,"AFC and FEI control","Modem PHY config"
    0x1B,RegAfcMsb,RegRfMode,0x00,0x00,"Frequency correction value of the AFC","Test register"
    0x1C,RegAfcLsb,RegHopPeriod,0x00,0x00,"Frequency correction value of the AFC","FHSS Hop period"
    0x1D,RegFeiMsb,RegNbRxBytes,0x00,0x00,"Value of the calculated","Number of received bytes"
    0x1E,RegFeiLsb,RegRxHeaderInfo,0x00,0x00,"frequency error","Info from last header"
    0x1F,RegPreambleDetect,RegRxHeaderCntValue,0x40,0xAA,"Settings of the Preamble Detector","Number of valid headers received"
    0x20,RegRxTimeout1,RegRxPacketCntValue,0x00,0x00,"Timeout Rx request and RSSI","Number of valid packets received"
    0x21,RegRxTimeout2,RegModemStat,0x00,0x00,"Timeout RSSI and PayloadReady","Live LoRa modem status"
    0x22,RegRxTimeout3,RegPktSnrValue,0x00,0x00,"Timeout RSSI and SyncAddress","Espimation of last packet SNR"
    0x23,RegRxDelay,RegRssiValue,0x00,0x00,"Delay between Rx cycles","Current RSSI"
    0x24,RegOsc,RegPktRssiValue,0x05,0x07,"RC Oscillators Settings, CLKOUT frequency","RSSi of last packet"
    0x25,RegPreambleMsb,RegHopChannel,0x00,0x00,"Preamble length, MSB","FHSS start channel"
    0x26,RegPreambleLsb,RegRxDataAddr,0x03,0x03,"Preamble length, LSB","LoRa rx data pointer"
    0x27,RegSyncConfig,RESERVED,0x93,0x93,"Sync Word Recognition control","RESERVED"
    0x28,RegSyncValue1,RESERVED,0x55,0x01,"Sync Word byte 1","RESERVED"
    0x29,RegSyncValue2,RESERVED,0x55,0x01,"Sync Word byte 2","RESERVED"
    0x2A,RegSyncValue3,RESERVED,0x55,0x01,"Sync Word byte 3","RESERVED"
    0x2B,RegSyncValue4,RESERVED,0x55,0x01,"Sync Word byte 4","RESERVED"
    0x2C,RegSyncValue5,RESERVED,0x55,0x01,"Sync Word byte 5","RESERVED"
    0x2D,RegSyncValue6,RESERVED,0x55,0x01,"Sync Word byte 6","RESERVED"
    0x2E,RegSyncValue7,RESERVED,0x55,0x01,"Sync Word byte 7","RESERVED"
    0x2F,RegSyncValue8,RESERVED,0x55,0x01,"Sync Word byte 8","RESERVED"
    0x30,RegPacketConfig1,RESERVED,0x90,0x90,"Packet mode settings","RESERVED"
    0x31,RegPacketConfig2,RESERVED,0x40,0x40,"Packet mode settings","RESERVED"
    0x32,RegPayloadLength,RESERVED,0x40,0x40,"Payload length setting","RESERVED"
    0x33,RegNodeAdrs,RESERVED,0x00,0x00,"Node address","RESERVED"
    0x34,RegBroadcastAdrs,RESERVED,0x00,0x00,"Broadcast address","RESERVED"
    0x35,RegFifoThresh,RESERVED,0x0F,0x8F,"Fifo threshold, Tx start condition","RESERVED"
    0x36,RegSeqConfig1,RESERVED,0x00,0x00,"Top level Sequencer settings","RESERVED"
    0x37,RegSeqConfig2,RESERVED,0x00,0x00,"Top level Sequencer settings","RESERVED"
    0x38,RegTimerResol,RESERVED,0x00,0x00,"Timer 1 and 2 resolution control","RESERVED"
    0x39,RegTimer1Coef,RESERVED,0xF5,0xF5,"Timer 1 setting","RESERVED"
    0x3A,RegTimer2Coef,RESERVED,0x20,0x20,"Timer 2 setting","RESERVED"
    0x3B,RegImageCal,RESERVED,0x82,0x02,"Image calibration engine control","RESERVED"
    0x3C,RegTemp,RESERVED,NONE,NONE,"Temperature Sensor value","RESERVED"
    0x3D,RegLowBat,RESERVED,0x02,0x02,"Low Battery Indicator Settings","RESERVED"
    0x3E,RegIrqFlags1,RESERVED,0x80,0x80,"Status register: PLL Lock state, Timeout, RSSI","RESERVED"
    0x3F,RegIrqFlags2,RESERVED,0x40,0x40,"Status register: FIFO handling flags, Low Battery","RESERVED"
    0x40,RegDioMapping1,RegDioMapping1,0x00,0x00,"Mapping of pins DIO0 to DIO3","Mapping of pins DIO0 to DIO3"
    0x41,RegDioMapping2,RegDioMapping2,0x00,0x00,"Mapping of pins DIO4 and DIO5,","Mapping of pins DIO4 and DIO5,"
    0x42,RegVersion,RegVersion,0x11,0x11,"Hope RF ID relating the silicon revision","Hope RF ID relating the silicon revision"
    0x44,RegPllHop,UNUSED,0x2D,0x2D,"Control the fast frequency hopping mode","Control the fast frequency hopping mode"
    0x4B,RegTcxo,RegTcxo,0x09,0x09,"TCXO or XTAL input setting","TCXO or XTAL input setting"
    0x4D,RegPaDac,RegPaDac,0x84,0x84,"Higher power settings of the PA","Higher power settings of the PA"
    0x5B,RegFormerTemp,RegFormerTemp,None,None,"Stored temperature during the former IQ Calibration","Stored temperature during the former IQ Calibration"
    0x5D,RegBitRateFrac,UNUSED,0x00,0x00,"Fractional part in the Bit Rate division ratio","UNUSED"
    0x61,RegAgcRef,RegAgcRef,0x13,0x13,"Adjustment of the AGC thresholds","Adjustment of the AGC thresholds"
    0x62,RegAgcThresh1,RegAgcThresh1,0x0E,0x0E,"Adjustment of the AGC thresholds","Adjustment of the AGC thresholds"
    0x63,RegAgcThresh2,RegAgcThresh2,0x5B,0x5B,"Adjustment of the AGC thresholds","Adjustment of the AGC thresholds"
    0x64,RegAgcThresh3,RegAgcThresh3,0xDB,0xDB,"Adjustment of the AGC thresholds","Adjustment of the AGC thresholds"
}