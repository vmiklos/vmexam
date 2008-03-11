import vobject, time, sys

sys = reload(sys)
sys.setdefaultencoding("latin2")

sock = sys.stdin
cal = vobject.readOne(sock)

items = {}
litems = ['ATTENDEE']

for i in litems:
	items[i] = []

for i in cal.vevent.lines():
	if i.name in litems:
		items[i.name].append(i)
	else:
		items[i.name] = i

print items['SUMMARY'].value
print
print "Time"
print "%s - %s" % (time.strftime("%H.%M", items['DTSTART'].value.timetuple()),
		time.strftime("%H.%M", items['DTEND'].value.timetuple()))
print
print "Date"
print time.strftime("%Y. %m. %d.", items['DTSTART'].value.timetuple())
print
print "Description"
print items['DESCRIPTION'].value
print
print "Organizer"
print items['ORGANIZER'].params['CN'][0]
print
print "Attendees"
for i in items['ATTENDEE']:
	print i.params['CN'][0]
print
print "Created"
print time.strftime("%Y. %m. %d. %H.%M.", items['CREATED'].value.timetuple())
