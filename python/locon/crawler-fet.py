#!/usr/bin/env python
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
#

from __future__ import print_function
from sgmllib import SGMLParser
import cgi
import time


class FETParser(SGMLParser):
    """Parses HTML generated by FET."""
    def reset(self):
        SGMLParser.reset(self)
        self.inRoom = False
        self.currentEvent = Event()
        self.currentData = []
        self.inStartTime = False
        self.inTrack = False
        self.inPerson = False
        self.inTitle = False
        self.events = []
        self.currentDay = 0

    def start_th(self, attrs):
        for k, v in attrs:
            if k == "colspan" and v == "3":
                self.inRoom = True
            elif k == "class" and v == "yAxis":
                self.inStartTime = True

    def end_th(self):
        if self.inRoom:
            self.inRoom = False
            self.currentEvent.room = "".join(self.currentData)
            self.currentData = []
        elif self.inStartTime:
            self.currentEvent.startTime = "".join(self.currentData)
            self.currentData = []
            self.inStartTime = False

    def start_td(self, attrs):
        for k, v in attrs:
            if k == "rowspan":
                self.currentEvent.timeMultiplier = int(v)
        self.currentDay += 1

    def end_td(self):
        if self.currentEvent.title:
            self.currentEvent.startDay = self.currentDay - 1
            self.currentEvent.countDuration()
            self.events.append(self.currentEvent)

            room = self.currentEvent.room
            startTime = self.currentEvent.startTime
            startDay = self.currentEvent.startDay
            self.currentEvent = Event()
            self.currentEvent.room = room
            self.currentEvent.startTime = startTime
            self.currentEvent.startDay = startDay
        self.currenTimeMultiplier = 1

    def start_tr(self, attrs):
        self.currentDay = 0

    def end_tr(self):
        room = self.currentEvent.room
        self.currentEvent = Event()
        self.currentEvent.room = room

    def start_div(self, attrs):
        for k, v in attrs:
            if k == "class" and "studentsset" in v:
                self.inTrack = True
            elif k == "class" and "teacher" in v:
                self.inPerson = True
            elif k == "class" and v == "line3":
                self.inTitle = True

    def end_div(self):
        if self.inTrack:
            self.inTrack = False
            self.currentEvent.track = "".join(self.currentData)
            self.currentData = []
        elif self.inPerson:
            self.inPerson = False
            self.currentEvent.person = "".join(self.currentData)
            self.currentData = []
        elif self.inTitle:
            self.inTitle = False
            self.currentEvent.title = "".join(self.currentData)
            self.currentData = []

    def handle_data(self, text):
        if self.inRoom or self.inStartTime or self.inTrack or self.inPerson or self.inTitle:
            self.currentData.append(text)

    def handle_comment(self, comment):
        if comment == " span ":
            self.currentDay += 1


class Event:
    """An event is a single talk."""
    def __init__(self):
        self.__timeSlots = [
            "09:00",
            "09:30",
            "10:00",
            "10:30",
            "10:50",
            "11:00",
            "11:30",
            "11:50",
            "12:00",
            "12:10",
            "12:30",
            "13:00",
            "13:30",
            "14:00",
            "14:30",
            "15:00",
            "15:30",
            "16:00",
            "16:30",
            "17:00",
            "17:30",
            "18:00",
            "18:30"
        ]
        self.room = None
        self.startTime = None
        self.track = None
        self.person = None
        self.timeMultiplier = 1
        self.title = None

    def __indexOf(self, l, item):
        """Returns the index of item in l."""
        for index, s in enumerate(l):
            if s == item:
                return index

    def __toHourMin(self, n):
        """Converts a duration in minutes to hh:mm string."""
        h = n / 60
        m = n % 60
        return "%d:%02d" % (h, m)

    def __toSec(self, s):
        """Converts 'hh:mm' string to seconds."""
        tokens = s.split(':')
        return int(tokens[0]) * 60 + int(tokens[1])

    def countDuration(self):
        """Count duration from startTime and timeMultiplier."""
        startPos = self.__indexOf(self.__timeSlots, self.startTime)
        endPos = startPos + self.timeMultiplier
        self.duration = self.__toHourMin(self.__toSec(self.__timeSlots[endPos]) - self.__toSec(self.__timeSlots[startPos]))


class Schedule:
    """The scedule is our document model: can be loaded and printed."""
    def __init__(self):
        self.events = []

    def loadFET(self, path):
        sock = open(path)
        fetHTML = sock.read()
        sock.close()

        parser = FETParser()
        parser.reset()
        parser.feed(fetHTML)
        parser.close()

        self.events = parser.events

    def printXML(self):
        print('<?xml version="1.0"?>')
        print('<schedule>')
        print('<conference>')
        print('<title>LibreOffice Conference 2015</title>')
        print('<city>Aarhus</city>')
        print('<start>2015-09-23</start>')
        print('<end>2015-09-25</end>')
        print('<days>3</days>')
        print('<release>1.%s</release>' % time.strftime("%Y%m%d.%H%M"))
        print('<timeslot_duration>00:10</timeslot_duration>')
        print('</conference>')

        eventId = 0
        for day in sorted(set([i.startDay for i in self.events])):
            print('<day date="2015-09-0%s" index="%s">' % (3 + day, day))
            for room in sorted(set([i.room for i in self.events if i.startDay == day])):
                print('<room name="%s">' % cgi.escape(room))
                for event in [i for i in self.events if i.startDay == day and i.room == room]:
                    print('<event id="%s">' % eventId)
                    eventId += 1
                    print('<start>%s</start>' % event.startTime)
                    print('<duration>%s</duration>' % event.duration)
                    print('<title>%s</title>' % cgi.escape(event.title))
                    print('<track>%s</track>' % cgi.escape(event.track))
                    print('<persons><person>%s</person></persons>' % cgi.escape(event.person))
                    print('<abstract>%s</abstract>' % cgi.escape(event.track))
                    print('</event>')
                print('</room>')
            print('</day>')
        print('</schedule>')

schedule = Schedule()
# http://conference.libreoffice.org/2014/program/
schedule.loadFET("index.html")
schedule.printXML()

# vim:set shiftwidth=4 softtabstop=4 expandtab:
