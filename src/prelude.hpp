#pragma once

#include "Arduino.h"
#include "ArduinoEigenDense.h"

// Shorter types.
typedef uint32_t u32;
typedef uint32_t u24;
typedef uint16_t u16;
typedef uint8_t u8;

typedef decltype([](u8 val){}) u8consumer;
typedef decltype([](u16 val){}) u16consumer;
typedef decltype([](u24 val){}) u24consumer;
typedef decltype([](u32 val){}) u32consumer;

typedef float_t f32;
typedef double_t f64;

/// @brief Size of a pointer or memory address on the flash chip.
typedef u24 flashaddr;

/// @brief Size of a pointer or memory address.
typedef decltype(sizeof(void*)) addr;