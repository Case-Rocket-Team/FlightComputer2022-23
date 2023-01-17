#include "log.hpp"
#include "prelude.hpp"

void write(u8 byte) {
    // TODO
}

void write_ptr(void* ptr, u8 length) {
    for (u8 i = 0; i < length; i++) {
        write(*(((u8*) ptr) + i));
    }
}

template <typename T>
void log(T data, String* annotationPtr) {
    log(data);
    unsigned int length = (*annotationPtr).length();
    while (length > 0) {
        u8 partLength = min(length, 255);
        write(LOG_ANNOTATION_PACKET_ID);
        write_ptr(annotationPtr, partLength);
        annotationPtr += partLength;
        length -= partLength;
    }
};

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