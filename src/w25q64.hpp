#pragma once

#include "Arduino.h"

class W25Q64 {
    public:
        static constexpr uint8_t WRITEENABLE      = 0x06;
        static constexpr uint8_t WRITEDISABLE     = 0x04;
        static constexpr uint8_t BLOCKERASE_4K    = 0x20;
        static constexpr uint8_t BLOCKERASE_32K   = 0x52;
        static constexpr uint8_t BLOCKERASE_64K   = 0xD8;
        static constexpr uint8_t CHIPERASE        = 0x60;

        static constexpr uint8_t STATUSREAD       = 0x05;
        static constexpr uint8_t STATUSWRITE      = 0x01;
        static constexpr uint8_t ARRAYREAD        = 0x0B;
        static constexpr uint8_t ARRAYREADLOWFREQ = 0x03;
        static constexpr uint8_t SLEEP            = 0xB9;
        static constexpr uint8_t WAKE             = 0xAB;
        static constexpr uint8_t BYTEPAGEPROGRAM  = 0x02;
        static constexpr uint8_t IDREAD           = 0x9F;
        static constexpr uint8_t MANUFAC          = 0x90;

        uint8_t flash_cs_pin;

        bool is_busy();

        struct ManufacDeviceRes {
            byte manufac;
            byte device;

            ManufacDeviceRes(uint16_t i) {
                manufac = (i & 0xff00) >> 2;
                device = i & 0xff;
            } 
        };
        
        ManufacDeviceRes manufac_and_device();

    private:
        template <typename T>
        T transact_now(T fn());
        
        template <typename T>
        T transact_when_ready(T fn());
};
