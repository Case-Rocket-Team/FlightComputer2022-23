#include <SPI.h>
#include <Arduino.h>
#include <LoRa.h>

class Tranciever {
public:
    const int csPin = 7;          // LoRa radio chip select (NSS)
    const int resetPin = 6;       // LoRa radio reset (NRESET)
    const int irqPin = 1;         // change for your board; must be a hardware interrupt pin (DIO0)
                                  // irqPin sends interrupts for onTxDone() and onRecieve()

    const long frequency = 915E6; // LoRa Frequency

    int msgCount = 0;
    long lastSendTime = 0;
    int interval = 2000;

    void setup();
    void loop();

    void sendMessage(String outgoing);
};