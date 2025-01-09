//////////////////////////////////////////////////////////////////////////////
// Author: Your Name
// Date: January 8, 2025
// About:
// This program serves as the Master in an I²C communication system for controlling 
// 12 servos on a robotic dog. The master first sends a practice DataPacket in the
// setup phase to confirm communication with the slave (multi-controller). It only 
// proceeds to the main loop after receiving confirmation.
//
// Other Important Info:
// - The practice packet does not affect servo angles on the slave.
// - The master ensures fault-tolerant initialization before entering main operation.
// - Ensure proper pull-up resistors (4.7kΩ or 10kΩ) are used on the SDA and SCL lines.
//////////////////////////////////////////////////////////////////////////////

#include <Wire.h>
#include <Arduino.h>

#define SLAVE_ADDRESS 8

// Shared data structure
struct DataPacket {
    int servoAngles[12]; // Angles for 12 servos (only used in actual packet)
    int confirmation;    // Confirmation flag (1 = success, 0 = failure)
};

// Function prototypes
bool sendAndConfirmPracticePacket();
void sendPracticePacket();
void sendData(DataPacket &data);
void receiveData(DataPacket &data);

// Global instances of DataPacket
DataPacket practicePacket = {
    {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}, // Unused angles (dummy)
    0                                     // Confirmation
};

DataPacket actualPacket = {
    {90, 45, 120, 90, 45, 120, 90, 45, 120, 90, 45, 120}, // Actual angles
    0                                                    // Confirmation
};

void setup() {
    Wire.begin(21, 22); // SDA, SCL pins for ESP32
    Serial.begin(115200);
    delay(1000);

    // Step 1: Send the practice packet
    Serial.println("Sending practice packet to Slave...");
    while (!sendAndConfirmPracticePacket()) {
        Serial.println("Error: Slave did not confirm practice packet. Retrying...");
        delay(1000); // Retry every second
    }

    Serial.println("Practice packet confirmed. Slave is ready.");
}

void loop() {
    // Step 2: Send the actual packet
    Serial.println("Sending actual packet to Slave...");
    sendData(actualPacket);

    // Step 3: Wait for confirmation from the slave
    delay(100); // Allow Slave to process
    receiveData(actualPacket);

    if (actualPacket.confirmation == 1) {
        Serial.println("Actual packet confirmed. All servos moved successfully.");
    } else {
        Serial.println("Error: Slave did not confirm actual packet.");
    }

    delay(2000); // Wait before sending the next command
}

// Function to send and confirm the practice packet
bool sendAndConfirmPracticePacket() {
    sendPracticePacket(); // Send the practice packet
    delay(100);           // Allow Slave to process
    receiveData(practicePacket);

    // Return true if confirmation is received
    return (practicePacket.confirmation == 1);
}

// Function to send the practice packet (doesn't affect servo angles)
void sendPracticePacket() {
    Wire.beginTransmission(SLAVE_ADDRESS);
    Wire.write((byte *)&practicePacket, sizeof(DataPacket)); // Send dummy packet
    if (Wire.endTransmission() != 0) {
        Serial.println("Error: Transmission of practice packet failed!");
    } else {
        Serial.println("Practice packet sent successfully.");
    }
}

// Function to send the actual data packet
void sendData(DataPacket &data) {
    Wire.beginTransmission(SLAVE_ADDRESS);
    Wire.write((byte *)&data, sizeof(DataPacket)); // Send struct as bytes
    if (Wire.endTransmission() != 0) {
        Serial.println("Error: Transmission of actual packet failed!");
    } else {
        Serial.println("Actual packet sent successfully.");
    }
}

// Function to receive the data packet from the Slave
void receiveData(DataPacket &data) {
    Wire.requestFrom(SLAVE_ADDRESS, sizeof(DataPacket)); // Request struct size
    if (Wire.available() == sizeof(DataPacket)) {
        Wire.readBytes((byte *)&data, sizeof(DataPacket)); // Read struct bytes
    } else {
        Serial.println("Error: Could not receive data.");
    }
}