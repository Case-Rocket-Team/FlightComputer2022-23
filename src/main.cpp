#include "Arduino.h"
#include "SPI.h"

// TODO: header files

void setup() {
    pinMode(FLASH_CS_PIN, OUTPUT);
    digitalWrite(FLASH_CS_PIN, HIGH);
    SPI.begin();
    Serial.begin(9600);
    Serial.println("Hello world!");
    Serial.println(manufac_and_device().manufac);
}

void loop() {
  
}