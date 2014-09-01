from sgmllib import SGMLParser
import time
import cgi

# room, startTime, track, person, title, timeMultiplier
class Event:
    def __init__(self):
        self.room = None
        self.startTime = None
        self.track = None
        self.person = None
        self.timeMultiplier = 1
        self.title = None

timeSlots = ["09:00", "09:30", "10:00", "10:30", "10:50", "11:00", "11:30", "11:50", "12:00", "12:10", "12:30", "13:00", "13:30", "14:00", "14:30", "15:00", "15:30", "16:00", "16:30", "17:00", "17:30", "18:00", "18:30"]

def get_schedule(path):
    class HTMLParser(SGMLParser):

        def reset(self):
            SGMLParser.reset(self)
            self.inTable = False
            self.inRoom = False
            self.current = Event()
            self.currentData = []
            self.inStartTime = False
            self.inTrack = False
            self.inPerson = False
            self.inTitle = False
            self.events = []

        def start_table(self, attrs):
            self.inTable = True

        def end_table(self):
            #print("debug, end of '%s' room" % self.currentRoom)
            self.current.room = None
            self.inTable = False

        def start_th(self, attrs):
            for k, v in attrs:
                if k == "colspan" and v == "3":
                    self.inRoom = True
                elif k == "class" and v == "yAxis":
                    self.inStartTime = True

        def end_th(self):
            if self.inRoom:
                self.inRoom = False
                self.current.room = "".join(self.currentData)
                self.currentData = []
                #print("debug, current room is '%s'" % self.current.room)
            elif self.inStartTime:
                self.current.startTime = "".join(self.currentData)
                self.currentData = []
                #print("debug, current start time is '%s'" % self.currentStartTime)
                self.inStartTime = False

        def start_td(self, attrs):
            for k, v in attrs:
                if k == "rowspan":
                    self.current.timeMultiplier = int(v)
            self.col += 1

        def end_td(self):
            def indexOf(l, item):
                for index, s in enumerate(l):
                    if s == item:
                        return index

            def to_sec(s):
                tokens = s.split(':')
                return int(tokens[0]) * 60 + int(tokens[1])

            def to_h_m(text):
                    n = int(text)
                    h = n / 60
                    m = n % 60
                    return "%d:%02d" % (h, m)

            if self.current.title:
                self.current.startDay = self.col - 1

                # handle duration
                #self.current.timeMultiplier
                startPos = indexOf(timeSlots, self.current.startTime)
                endPos = startPos + self.current.timeMultiplier
                #print("debug, timeSlots is '%s'" % timeSlots)
                #print("startPos is %d, endPos is %d" % (startPos, endPos))
                #self.current.duration = "%s-%s" % (timeSlots[startPos], timeSlots[endPos])
                self.current.duration = to_h_m(to_sec(timeSlots[endPos]) - to_sec(timeSlots[startPos]))

                #print("debug, event: room is '%s', start is '%s, %s', duration is '%s', title is '%s', track is '%s', person is '%s'" % (self.current.room, self.current.startDay, self.current.startTime, self.current.duration, self.current.title, self.current.track, self.current.person))
                self.events.append(self.current)

                room = self.current.room
                startTime = self.current.startTime
                startDay = self.current.startDay
                self.current = Event()
                self.current.room = room
                self.current.startTime = startTime
                self.current.startDay = startDay

                self.current.title = None
            self.currenTimeMultiplier = 1

        def start_tr(self, attrs):
            self.col = 0

        def end_tr(self):
            room = self.current.room
            self.current = Event()
            self.current.room = room

        def start_div(self, attrs):
            for k, v in attrs:
                if k == "class" and "studentsset" in v:
                    self.inTrack = True
                    #print("debug, now in track")
                elif k == "class" and "teacher" in v:
                    self.inPerson = True
                elif k == "class" and v == "line3":
                    self.inTitle = True

        def end_div(self):
            if self.inTrack:
                self.inTrack = False
                self.current.track = "".join(self.currentData)
                self.currentData = []
                #print("debug, current track is '%s'" % self.currentTrack)
            elif self.inPerson:
                self.inPerson = False
                self.current.person = "".join(self.currentData)
                self.currentData = []
                #print("debug, current person is '%s'" % self.currentPerson)
            elif self.inTitle:
                self.inTitle = False
                self.current.title = "".join(self.currentData)
                self.currentData = []
                #print("debug, current title is '%s'" % self.currentTitle)

        def handle_data(self, text):
            if self.inRoom or self.inStartTime or self.inTrack or self.inPerson or self.inTitle:
                #print("debug, got text portion '%s'" % text)
                self.currentData.append(text)
        
        def handle_comment(self, comment):
            if comment == " span ":
                self.col += 1
    
    sock = open(path)#urllib.urlopen(url)
    page = sock.read()
    sock.close()

    parser = HTMLParser()
    parser.reset()
    parser.feed(page)
    parser.close()

    print '<?xml version="1.0"?>'
    print '<schedule>'
    print '<conference>'
    print '<title>LibreOffice Conference 2014</title>'
    print '<city>Bern</city>'
    print '<start>2014-09-03</start>'
    print '<end>2014-09-05</end>'
    print '<days>3</days>'
    print '<release>1.%s</release>' % time.strftime("%Y%m%d.%H%M")
    print '<timeslot_duration>00:10</timeslot_duration>'
    print '</conference>'

    eventId = 0
    for day in sorted(set([i.startDay for i in parser.events])):
        print '<day date="2014-09-0%s" index="%s">' % (3 + day, day)
        for room in sorted(set([i.room for i in parser.events if i.startDay == day])):
            print '<room name="%s">' % cgi.escape(room)
            for event in [i for i in parser.events if i.startDay == day and i.room == room]:
                print '<event id="%s">' % eventId
                eventId += 1
                print '<start>%s</start>' % event.startTime
                print '<duration>%s</duration>' % event.duration
                print '<title>%s</title>' % cgi.escape(event.title)
                print '<track>%s</track>' % cgi.escape(event.track)
                print '<persons><person>%s</person></persons>' % cgi.escape(event.person)
                print '<abstract>%s</abstract>' % cgi.escape(event.track)
                print '</event>'
            print '</room>'
        print '</day>'

    print '</schedule>'

get_schedule("index.html")

# vim:set shiftwidth=4 softtabstop=4 expandtab:
