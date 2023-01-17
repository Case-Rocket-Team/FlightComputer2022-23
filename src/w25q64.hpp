#pragma once

#include "prelude.hpp"

class W25Q64 {
    public:
        static inline constexpr u8 WRITEENABLE      = 0x06;
        static inline constexpr u8 WRITEDISABLE     = 0x04;
        static inline constexpr u8 BLOCKERASE_4K    = 0x20;
        static inline constexpr u8 BLOCKERASE_32K   = 0x52;
        static inline constexpr u8 BLOCKERASE_64K   = 0xD8;
        static inline constexpr u8 CHIPERASE        = 0x60;

        static inline constexpr u8 STATUSREAD       = 0x05;
        static inline constexpr u8 STATUSWRITE      = 0x01;
        static inline constexpr u8 ARRAYREAD        = 0x0B;
        static inline constexpr u8 ARRAYREADLOWFREQ = 0x03;
        static inline constexpr u8 SLEEP            = 0xB9;
        static inline constexpr u8 WAKE             = 0xAB;
        static inline constexpr u8 BYTEPAGEPROGRAM  = 0x02;
        static inline constexpr u8 IDREAD           = 0x9F;
        static inline constexpr u8 MANUFAC          = 0x90;

        W25Q64(u8 cs_pin);
        u8 flash_cs_pin;

        struct ManufacDeviceRes {
            u8 manufac;
            u8 device;
        };
        
        bool is_busy();

        /// @brief  Blocks until the chip is ready.
        /// @return Returns the chip object (for chaining)
        W25Q64* ready();

        /// @brief Query the chip for the manufacturer & device info
        /// @return Manufacture and device
        ManufacDeviceRes manufac_and_device();

        /// @brief Enable writing on the chip. Automatically called
        /// before all write operations.
        void write_enable();

        /// @brief Write data from a pointer to the chip. Note:
        /// Length is bounded by the current page -- do not go
        /// past the current page.
        /// @param addr Destination address on the chip
        /// @param ptr Pointer to the data to write
        /// @param length Number of bytes to be written
        void write(uint32_t addr, void* ptr, u8 length);

        template <typename F>
        inline void read(uint32_t addr, uint32_t length, F&& byteHandler);

        class Transactor {
            public:
                Transactor* send8(u8 val);
                Transactor* send16(u16 val);
                Transactor* send24(u24 val);
                Transactor* send32(u32 val);
                u8 receive8(u8 val);
                u16 receive16(u16 val);
                u24 receive24(u24 val);
                u32 receive32(u32 val);
                u8 receive8();
                u16 receive16();
                u24 receive24();
                u32 receive32();
        };

    private:

        template <typename F>
        constexpr auto transact(F&& f) -> std::enable_if_t<!std::is_void_v<decltype(f(new Transactor()))>, decltype(f(new Transactor()))>;

        template <typename F>
        constexpr auto transact(F&& f) -> std::enable_if_t<std::is_void_v<decltype(f(new Transactor()))>>;
        
        Transactor* transactor = new Transactor();
};