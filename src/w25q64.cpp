#include "Arduino.h"
#include "SPI.h"
#include "functional"


#define FLASH_CS_PIN 10

#define WRITEENABLE      0x06
#define WRITEDISABLE     0x04
#define BLOCKERASE_4K    0x20
#define BLOCKERASE_32K   0x52
#define BLOCKERASE_64K   0xD8
#define CHIPERASE        0x60
                             
#define STATUSREAD       0x05
#define STATUSWRITE      0x01
#define ARRAYREAD        0x0B
#define ARRAYREADLOWFREQ 0x03
#define SLEEP            0xB9
#define WAKE             0xAB
#define BYTEPAGEPROGRAM  0x02
#define IDREAD           0x9F
#define MANUFAC          0x90

template <typename T>
T transact_now(T fn()) {
    SPI.beginTransaction(SPISettings(400000, MSBFIRST, SPI_MODE0));
    digitalWrite(FLASH_CS_PIN, LOW);
    T ret = fn();
    SPI.endTransaction();
    digitalWrite(FLASH_CS_PIN, HIGH);
    return ret;
}

bool is_busy() {
    return transact_now<bool>([](){ return (SPI.transfer(STATUSREAD) & 0x01) == 1; });
}

template <typename T>
T transact_when_ready(T fn()) {
    while (is_busy()) { };

    return transact_now(fn);
}

struct ManufacDeviceRes {
    byte manufac;
    byte device;

    ManufacDeviceRes(uint16_t i) {
        manufac = (i & 0xff00) >> 2;
        device = i & 0xff;
    } 
};

ManufacDeviceRes manufac_and_device() {
    return transact_when_ready<ManufacDeviceRes>([] { 
        SPI.transfer16(0x9000);
        SPI.transfer16(0x00);
        return (ManufacDeviceRes) SPI.transfer16(0);
    });
};