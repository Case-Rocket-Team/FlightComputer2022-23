#include "prelude.hpp"
#include "SPI.h"

#include "w25q64.hpp"

// Templates below helpfully written by ChatGPT.
// SPI and digital write lines added by Serena.
template <typename F>
constexpr auto W25Q64::transact(F&& f) -> std::enable_if_t<!std::is_void_v<decltype(f(new Transactor()))>, decltype(f(new Transactor()))>{
    SPI.beginTransaction(SPISettings(400000, MSBFIRST, SPI_MODE0));
    digitalWrite(flash_cs_pin, LOW);
    auto val = f(transactor);
    SPI.endTransaction();
    digitalWrite(flash_cs_pin, HIGH);
    return val;
};

template <typename F>
constexpr auto W25Q64::transact(F&& f) -> std::enable_if_t<std::is_void_v<decltype(f(new Transactor()))>, decltype(f(new Transactor()))>{
    SPI.beginTransaction(SPISettings(400000, MSBFIRST, SPI_MODE0));
    digitalWrite(flash_cs_pin, LOW);
    f(transactor);
    SPI.endTransaction();
    digitalWrite(flash_cs_pin, HIGH);
};
// End ChatGPT.

#define TRANSACT(fn) (transact([&](Transactor* t)fn))

u8 W25Q64::Transactor::receive8(u8 val) {
    return SPI.transfer(val);
}

u8 W25Q64::Transactor::receive8() {
    return receive8(0);
}

u16 W25Q64::Transactor::receive16(u16 val) {
    return SPI.transfer16(val);
}

u16 W25Q64::Transactor::receive16() {
    return receive16(0);
}

u24 W25Q64::Transactor::receive24(u24 val) {
    u24 num = 0;
    num += SPI.transfer(val >> 16) << 16;
    num += SPI.transfer(val >> 8) << 8;
    num += SPI.transfer(val);
    return num;
}

u24 W25Q64::Transactor::receive24() {
    return receive24(0);
}

u32 W25Q64::Transactor::receive32(u32 val) {
    return SPI.transfer32(val);
}

u32 W25Q64::Transactor::receive32() {
    return receive32(0);
}

W25Q64::Transactor* W25Q64::Transactor::send8(u8 val) {
    receive8(val);
    return this;
}

W25Q64::Transactor* W25Q64::Transactor::send16(u16 val) {
    receive16(val);
    return this;
}

W25Q64::Transactor* W25Q64::Transactor::send24(u24 val) {
    receive24(val);
    return this;
}

W25Q64::Transactor* W25Q64::Transactor::send32(u32 val) {
    receive32(val);
    return this;
}

W25Q64::W25Q64(u8 flash_cs_pin)  {
    this->flash_cs_pin = flash_cs_pin;
    
    pinMode(flash_cs_pin, OUTPUT);
    digitalWrite(flash_cs_pin, HIGH);
}

bool W25Q64::is_busy() {
    return TRANSACT({
        return (SPI.transfer(STATUSREAD) & 0x01) == 1;
    });
}

W25Q64* W25Q64::ready() {
    while (is_busy()) { };
    return this;
}

W25Q64::ManufacDeviceRes W25Q64::manufac_and_device() {
    return transact([&](Transactor* t){
        t->send8(0x90)->send24(0);
        return (ManufacDeviceRes) {
            .manufac = t->receive8(),
            .device = t->receive8()
        };
    });
};

void W25Q64::write_enable() {
    TRANSACT({
        SPI.transfer(0x06);
    });
}

void W25Q64::write(uint32_t addr, void* _ptr, u8 length) {
    write_enable();

    auto ptr = (u8*) _ptr;

    TRANSACT({
        SPI.transfer(0x02);
        SPI.transfer(addr >> 16);
        SPI.transfer(addr >> 8);
        SPI.transfer(addr);
        for (int i = 0; i < length; i++) {
            SPI.transfer(*ptr);
            ptr += 1;
        };
    });
}
/*
template <typename F>
void W25Q64::read(addr addr, uint32_t length, F&& fn) {
    TRANSACT({
        SPI.transfer(0x03);
        SPI.transfer(addr >> 16);
        SPI.transfer(addr >> 8);
        SPI.transfer(addr);

        for (int i = 0; i < length; i++) {
            fn(SPI.transfer(0x00));
        }
    });
};*/

/*
void W2564::transfer_address(uint32_t addr) {
    SPI.transfer(addr >> 16);
    SPI.transfer(addr >> 8);
    SPI.transfer(addr);

}

template <typename T>
void W25Q64::read(addr addr, T item) {
    TRANSACT({
        SPI.transfer(0x03);
        transfer_address(addr);
        
        for (int i = 0; i < length; i++) {
            fn(SPI.transfer(0x00));
        }
    })
};


void W25Q64::Transactor::send(u8... data) {
    for (u8 byte : { data... }) {
        SPI.transfer(byte);
    }
}*/

#undef TRANSACT