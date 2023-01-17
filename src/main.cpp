#include "prelude.hpp"
#include "SPI.h"
#include "state.hpp"
#include "subsystems/logger_subsystem.hpp"
#include "subsystem_manager.hpp"
#include "instrument_manager.hpp"
#include "w25q64.hpp"

// TODO: move the following declarations to their respective files
//contains the 3 axis acceleration
struct ImuAcceleration {
    //this fits better with the gps, we could also pick arbitrary axises
    //relative to the starting position
    long xAccel;
    long yAccel;
    long zAccel;
};

//contains the 3 axis rotational acceleration
struct ImuRotation{
    long xRot;
    long yRot;
    long zRot;
};
// end TODO

static InstrumentManager instruments = InstrumentManager {};
static DerivedState derived_state = DerivedState {};
static SubsystemManager subsystems = SubsystemManager {};
static W25Q64 flash(13);

void setup() {
    SPI.begin();
    Serial.begin(9600);

    Serial.println("Hello world!");
    Serial.println(flash.manufac_and_device().manufac);
}

void derive_state(InstrumentManager &instruments, DerivedState &derived_state) {
    // TODO
}

void loop() {
    instruments.poll();
    derive_state(instruments, derived_state);
    subsystems.apply(derived_state);
}