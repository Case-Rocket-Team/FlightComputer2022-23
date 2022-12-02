#include "Arduino.h"
#include "SPI.h"
#include "state.hpp"
#include "subsystems/logger_subsystem.hpp"
#include "subsystem_manager.hpp"
#include "instrument_manager.hpp"

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

void setup() {
    /*pinMode(FLASH_CS_PIN, OUTPUT);
    digitalWrite(FLASH_CS_PIN, HIGH);
    SPI.begin();
    Serial.begin(9600);
    Serial.println("Hello world!");
    Serial.println(manufac_and_device().manufac);*/
}

void loop() {
    instruments.poll();
    instruments.derive_state(derived_state);
    subsystems.apply(derived_state);
}