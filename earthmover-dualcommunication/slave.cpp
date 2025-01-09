//////////////////////////////////////////////////////////////////////////////
// Author: Christian Zastrow
// Date: January 8, 2025
// About:
// This program serves as the Slave in an I²C communication system for controlling
// 12 servos on a robotic dog. During setup, the servos are zeroed using limit switches 
// to ensure they start at a known position. The slave responds to a practice packet 
// from the master during setup to confirm communication readiness. Once confirmed, 
// the slave processes actual servo commands from the master.
//
// Other Important Info:
// - The slave uses limit switches to zero the servos during setup.
// - Proper pull-up resistors (4.7kΩ or 10kΩ) are required on SDA and SCL lines.
// - Designed for fault-tolerant and reliable communication.
//////////////////////////////////////////////////////////////////////////////

#include <Wire.h>
#include <Servo.h>
#include <Arduino.h>

// Define structs
struct Leg {
    int hipServo;
    int kneeServo;
    int ankleServo;
    int hipLimitSwitch;
    int kneeLimitSwitch;
    int ankleLimitSwitch;
};

struct DataPacket {
    int servoAngles[12]; // Angles for 12 servos
    int confirmation;    // Confirmation flag (1 = success, 0 = failure)
};

// Function prototypes
void sendData();
void receiveData(int bytes);
void zeroServos(Leg leg, int servoIndex);
bool isPracticePacket(const DataPacket &packet);

// Initialize legs with servo pins and limit switch pins
Leg frontLeft = {2, 3, 4, 22, 23, 24};  // Pins for servos and limit switches
Leg frontRight = {5, 6, 7, 25, 26, 27};
Leg backLeft = {8, 9, 10, 28, 29, 30};
Leg backRight = {11, 12, 13, 31, 32, 33};

Servo servos[12]; // Array to manage 12 servo objects
DataPacket packet = {
    {90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90}, // Default angles
    0                                                // Confirmation
};

void setup() {
    Wire.begin(8); // Initialize as Slave
    Wire.onReceive(receiveData); // Register receive handler
    Wire.onRequest(sendData);    // Register request handler

    Serial.begin(115200);

    // Attach all 12 servos to pins
    servos[0].attach(frontLeft.hipServo);
    servos[1].attach(frontLeft.kneeServo);
    servos[2].attach(frontLeft.ankleServo);
    servos[3].attach(frontRight.hipServo);
    servos[4].attach(frontRight.kneeServo);
    servos[5].attach(frontRight.ankleServo);
    servos[6].attach(backLeft.hipServo);
    servos[7].attach(backLeft.kneeServo);
    servos[8].attach(backLeft.ankleServo);
    servos[9].attach(backRight.hipServo);
    servos[10].attach(backRight.kneeServo);
    servos[11].attach(backRight.ankleServo);

    // Set limit switch pins as inputs
    pinMode(frontLeft.hipLimitSwitch, INPUT_PULLUP);
    pinMode(frontLeft.kneeLimitSwitch, INPUT_PULLUP);
    pinMode(frontLeft.ankleLimitSwitch, INPUT_PULLUP);

    pinMode(frontRight.hipLimitSwitch, INPUT_PULLUP);
    pinMode(frontRight.kneeLimitSwitch, INPUT_PULLUP);
    pinMode(frontRight.ankleLimitSwitch, INPUT_PULLUP);

    pinMode(backLeft.hipLimitSwitch, INPUT_PULLUP);
    pinMode(backLeft.kneeLimitSwitch, INPUT_PULLUP);
    pinMode(backLeft.ankleLimitSwitch, INPUT_PULLUP);

    pinMode(backRight.hipLimitSwitch, INPUT_PULLUP);
    pinMode(backRight.kneeLimitSwitch, INPUT_PULLUP);
    pinMode(backRight.ankleLimitSwitch, INPUT_PULLUP);

    // Zero all servos using limit switches
    Serial.println("Zeroing servos...");
    zeroServos(frontLeft, 0);
    zeroServos(frontRight, 3);
    zeroServos(backLeft, 6);
    zeroServos(backRight, 9);

    // Confirm zeroing is complete
    packet.confirmation = 1; // Indicate success
    Serial.println("All servos zeroed. Ready to receive commands.");
}

void loop() {
    delay(100); // Keep loop lightweight for I²C
}

// Function to zero servos using limit switches
void zeroServos(Leg leg, int servoIndex) {
    // Zero hip servo
    while (digitalRead(leg.hipLimitSwitch) == HIGH) {
        servos[servoIndex].write(0); // Move toward the limit switch
        delay(10);
    }
    servos[servoIndex].write(90); // Move to default position
    servoIndex++;

    // Zero knee servo
    while (digitalRead(leg.kneeLimitSwitch) == HIGH) {
        servos[servoIndex].write(0); // Move toward the limit switch
        delay(10);
    }
    servos[servoIndex].write(90); // Move to default position
    servoIndex++;

    // Zero ankle servo
    while (digitalRead(leg.ankleLimitSwitch) == HIGH) {
        servos[servoIndex].write(0); // Move toward the limit switch
        delay(10);
    }
    servos[servoIndex].write(90); // Move to default position
}

// Function to determine if a packet is a practice packet
bool isPracticePacket(const DataPacket &packet) {
    for (int i = 0; i < 12; i++) {
        if (packet.servoAngles[i] != 0) {
            return false; // If any servo angle is not zero, it's not a practice packet
        }
    }
    return true;
}

// Function to receive data from Master
void receiveData(int bytes) {
    if (bytes == sizeof(DataPacket)) {
        Wire.readBytes((byte *)&packet, sizeof(DataPacket)); // Read struct

        if (isPracticePacket(packet)) {
            // Respond to practice packet
            Serial.println("Practice packet received. Confirming readiness...");
            packet.confirmation = 1; // Confirm readiness
        } else {
            // Process actual packet
            Serial.println("Servo angles received from Master:");
            for (int i = 0; i < 12; i++) {
                servos[i].write(packet.servoAngles[i]);
                Serial.print("Servo ");
                Serial.print(i);
                Serial.print(" moved to ");
                Serial.println(packet.servoAngles[i]);
            }
            packet.confirmation = 1; // Confirm success
            Serial.println("All servos moved successfully.");
        }
    } else {
        Serial.println("Error: Received data size mismatch.");
    }
}

// Function to send data to Master
void sendData() {
    Wire.write((byte *)&packet, sizeof(DataPacket)); // Send updated struct
    Serial.println("Confirmation sent to Master.");
}
