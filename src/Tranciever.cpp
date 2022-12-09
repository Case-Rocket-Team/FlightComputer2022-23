#include <Tranciever.hpp>

void Tranciever::setup(){
    Serial.println("LoRa Tranciever");

    LoRa.setPins(csPin, resetPin, irqPin);

    if(!LoRa.begin(frequency)){
        Serial.println("Starting LoRa failed!");
    }
}

void Tranciever::loop(){
    // try to parse packet
    int packetSize = LoRa.parsePacket();
    if (packetSize) {
        // received a packet
        Serial.print("Received packet '");

        // read packet byte by byte
        while (LoRa.available()) {
            Serial.print((char)LoRa.read());
        }

        // print RSSI of packet
        Serial.print("' with RSSI ");
        Serial.println(LoRa.packetRssi());
    }

    //send a message every 2 seconds
    if (millis() - lastSendTime > interval) {
        String message = "HeLoRa World!";   // send a message
        sendMessage(message);
        Serial.println("Sending " + message);
        lastSendTime = millis();            // timestamp the message
    }
}

void Tranciever::sendMessage(String outgoing){
    LoRa.beginPacket();
    // write adds a single byte to the packet
    LoRa.write(msgCount);
    // print adds multiple bytes to the packet
    LoRa.print(outgoing);
    LoRa.endPacket();

    msgCount++;
}