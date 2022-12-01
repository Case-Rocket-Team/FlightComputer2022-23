#include "Arduino.h"
#include "SPI.h"
#include "state.hpp"
#include "subsystems/logger_subsystem.hpp"
#include "subsystem_manager.hpp"
#include "instrument_manager.hpp"

// TODO: header files

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

//create data structures to store data
struct RawState {
    //baro
    long baro;
    //imu
    ImuAcceleration accel;
    ImuRotation rot;
    
    //GPS
    long lat;
    long lng;
    long vel;
    long alt;
};


static RawState raw_state = RawState {};
static DerivedState derived_state = DerivedState {};
static SubsystemManager subsystems = SubsystemManager {};
static InstrumentManager instruments = InstrumentManager {};

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