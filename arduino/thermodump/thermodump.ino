/*
 * Copyright 2018 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

#include <DHT.h>
#include <RTClib.h>
#include <SD.h>
#ifdef THERMODUMP_CONFIG
#include <SDConfigFile.h>
#endif
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

File csv;

RTC_DS1307 rtc;
// Default value in ms in case of no config on the sd card.
int delayMS = 10000;

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

#ifdef THERMODUMP_CONFIG
    SDConfigFile config;
    const uint8_t configLineLength = 127;
    const char* configFile = "thermo.cfg";
    if (!config.begin(configFile, configLineLength))
    {
        Serial.println("setup: sd config failed");
        return;
    }

    while (config.readNextSetting())
    {
        if (config.nameIs("delay"))
        {
            delayMS = config.getIntValue();
            Serial.print("setup, sd config: delay is ");
            Serial.print(delayMS);
            Serial.println(".");
        }
    }
    Serial.println("setup: sd config done");
#endif

    Wire.begin();
    rtc.begin();
    if (!rtc.isrunning())
    {
        Serial.println("setup: rtc init");
        // This could be made configurable similar to delayMS.
        rtc.adjust(DateTime(__DATE__, __TIME__));
    }
    Serial.println("setup: rtc done");

    pinMode(ledPin, OUTPUT);
    Serial.println("setup: led done");
}

void loop()
{
    // Pause between two measures.
    delay(delayMS);

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
    csv = SD.open(csvName, FILE_WRITE);
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
