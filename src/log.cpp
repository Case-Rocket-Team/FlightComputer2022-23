#include "Arduino.h"


void write(uint8_t byte) {
    // TODO
}

void write_ptr(void * ptr, uint8_t length) {
    for (uint8_t i = 0; i < length; i++) {
        write(*(((uint8_t*) ptr) + i));
    }
}

void log(String* msg) {
    auto length = (*msg).length();
    write(0x00);
    write(length);
    write_ptr(msg, length);
}