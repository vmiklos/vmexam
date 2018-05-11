/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#include <DHT.h>
#include <RTClib.h>
#include <SD.h>
#include <Wire.h>

/**
 * The HW has these parts:
 * - Serial debug console
 * - Thermo sensor (DHT)
 * - SD card reader
 * - Clock (RTC)
 * - LED
 */

// The AM2302/AM2321 thermo sensor is connected to D3.
DHT dht(/*pin=*/3, /*type=*/DHT22);

RTC_DS1307 rtc;

// The led is connected to D5.
constexpr int ledPin = 5;

void setup()
{
    Serial.begin(9600);
    while (!Serial)
    {
        // Wait for serial port to connect. Needed for native USB port only.
    }
    Serial.println("setup: serial done");

    dht.begin();
    Serial.println("setup: dht done");

    if (!SD.begin(4))
    {
        Serial.println("setup: sd failed");
        return;
    }
    Serial.println("setup: sd done");

    Wire.begin();
    rtc.begin();
    if (!rtc.isrunning())
    {
        Serial.println("setup: rtc init");
        // TODO make this configurable.
        rtc.adjust(DateTime(__DATE__, __TIME__));
    }
    Serial.println("setup: rtc done");

    pinMode(ledPin, OUTPUT);
    Serial.println("setup: led done");
}

void loop()
{
    // Pause between two measures.
    // TODO make this configurable.
    delay(2000);

    // Led on.
    digitalWrite(ledPin, HIGH);

    // Get temperature.
    float temperature = dht.readTemperature();
    if (isnan(temperature))
    {
        Serial.println("loop, dht: readTemperature() failed");
        return;
    }

    // Get time.
    DateTime now = rtc.now();

    // Write them out to the SD card.
    const char* csvName = "thermo.csv";
    File csv = SD.open(csvName, FILE_WRITE);
    if (!csv)
    {
        Serial.println("loop, sd: failed to open");
        return;
    }

    Serial.print("loop, sd: writing...");
    csv.print(now.year(), DEC);
    csv.print('-');
    csv.print(now.month(), DEC);
    csv.print('-');
    csv.print(now.day(), DEC);
    csv.print(' ');
    csv.print(now.hour(), DEC);
    csv.print(':');
    csv.print(now.minute(), DEC);
    csv.print(':');
    csv.print(now.second(), DEC);
    csv.print(',');
    csv.println(temperature);
    csv.close();
    Serial.println(" done");

    // Led off.
    digitalWrite(ledPin, LOW);
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
