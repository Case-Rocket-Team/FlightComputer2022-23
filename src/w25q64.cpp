#include "Arduino.h"
#include "SPI.h"
#include "functional"

#include "w25q64.hpp"

template <typename T>
T W25Q64::transact_now(T fn()) {
    SPI.beginTransaction(SPISettings(400000, MSBFIRST, SPI_MODE0));
    digitalWrite(flash_cs_pin, LOW);
    T ret = fn();
    SPI.endTransaction();
    digitalWrite(flash_cs_pin, HIGH);
    return ret;
};

bool W25Q64::is_busy() {
    return transact_now<bool>([](){ return (SPI.transfer(STATUSREAD) & 0x01) == 1; });
}

template <typename T>
T W25Q64::transact_when_ready(T fn()) {
    while (is_busy()) { };

    return transact_now(fn);
};

W25Q64::ManufacDeviceRes W25Q64::manufac_and_device() {
    return transact_when_ready<ManufacDeviceRes>([] { 
        SPI.transfer16(0x9000);
        SPI.transfer16(0x00);
        return (ManufacDeviceRes) SPI.transfer16(0);
    });
};