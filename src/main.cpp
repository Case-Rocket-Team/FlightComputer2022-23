#include "Arduino.h"
#include "SPI.h"
#include "state.hpp"
#include "subsystems/logger_subsystem.hpp"
#include "subsystem_manager.hpp"

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

void setup() {
    /*pinMode(FLASH_CS_PIN, OUTPUT);
    digitalWrite(FLASH_CS_PIN, HIGH);
    SPI.begin();
    Serial.begin(9600);
    Serial.println("Hello world!");
    Serial.println(manufac_and_device().manufac);*/
}

void loop() {
    //pollInstrumentsInto(raw_state);
    //deriveState(raw_state, derived_state);
    subsystems.apply(derived_state);
}

/// @brief WRITES data into rawState by Reading from instruments
/// @param rawState the raw state object 
/// @return true if write sucessful?? 
/*boolean pollInstrumentsInto(RawState &raw) {
    
    // barometer
    //IMU
    //gps
    
}*/

/// @brief WRITES data into derivedState with info form rawstate
/// @param raw the raw state
/// @param derived the derived state
/// @return true if writing is sucessful
/*boolean deriveState(RawState& raw, DerivedState& derived){

    //raw (some)
    
}*/

/// @brief drives the motors based on the derived state
/// @param state , the input derived state
/// @return true if motors were sucesfully driven, otherwise false
boolean drive_motors(DerivedState state){
    
}

/// @brief drives the pyros based on the derived state
/// @param state , the input derived state
/// @return true if pyros were sucesfully driven, otherwise false
boolean drive_pyros(DerivedState state){
    
}