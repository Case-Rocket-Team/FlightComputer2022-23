#include "log.hpp"
#include "Arduino.h"

void write(uint8_t byte) {
    // TODO
}

void write_ptr(void* ptr, uint8_t length) {
    for (uint8_t i = 0; i < length; i++) {
        write(*(((uint8_t*) ptr) + i));
    }
}

template <typename T>
void log(T data, String* annotation) {
    log(data);
    int length = annotation.length();
    while (length > 0) {
        int partLength = min(length, 255);
        write(LOG_ANNOTATION_PACKET_ID);
        write_ptr(annotation, partLength);
        annotation += partLength;
        length -= partLength
    }
}

void log(String* msg) {
    auto length = (*msg).length();
    write(LOG_NULL_PACKET_ID);
    write(LOG_ANNOTATION_PACKET_ID);
    write(length);
    write_ptr(msg, length);
}

void log(int32_t* num) {
    write(LOG_I32_PACKET_ID);
    write_ptr(num, 4);
}

void log(float_t* num) {
    write(LOG_F32_PACKET_ID);
    write_ptr(num, 4);
}

void log(double_t* num) {
    write(LOG_F64_PACKET_ID);
    write_ptr(num, 8);
}