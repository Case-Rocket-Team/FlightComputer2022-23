#pragma once

#define LOG_NULL_PACKET_ID          0x00
#define LOG_ANNOTATION_PACKET_ID    0x01
#define LOG_I32_PACKET_ID           0x02
#define LOG_F32_PACKET_ID           0x03
#define LOG_F64_PACKET_ID           0x04

#include <Arduino.h>

template <typename T>
void log(T data);

template <typename T>
void log(T data, String* annotation);