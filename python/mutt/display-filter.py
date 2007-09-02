#!/usr/bin/env python

import time, rfc822, sys
from email.Utils import formatdate

def get_zone():
	now = time.localtime()
	if time.daylight and now[-1]:
		offset = time.altzone
	else:
		offset = time.timezone
	hours, minutes = divmod(abs(offset), 3600)
	if offset > 0:
		sign = '-'
	else:
		sign = '+'
	return '%s%02d%02d' % (sign, hours, minutes // 60)

def improve_date(input):
	mytz = input.split(" ")[-1].strip()
	if mytz[1:-1] in time.tzname or mytz == get_zone():
		return input
	else:
		tz = rfc822.parsedate_tz(input)
		return "%s (%s)" % (formatdate(time.mktime(tz[:9])-tz[9]-(time.timezone), True), input)

lines = sys.stdin.readlines()

for i in lines:
	if i.startswith("Date: "):
		date = i[6:-1]
		o = "Date: %s\n" % improve_date(date)
	else:
		o = i
	sys.stdout.write(o)
