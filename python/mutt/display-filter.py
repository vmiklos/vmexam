#!/usr/bin/env python3

import time, sys, re
import email.utils
import io

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
		tz = email.utils.parsedate_tz(input)
		if not tz or not tz[9]:
			return input
		return "%s (%s)" % (email.utils.formatdate(time.mktime(tz[:9])-tz[9]-(time.timezone), True), input)

# Tolerate in case the input is not valid utf-8.
stdin = sys.stdin.buffer.read().decode("utf-8", errors="replace")
stdin_stream = io.StringIO(stdin)
lines = stdin_stream.readlines()

o = []

for i in lines:
	# add local date _as well_ if the timezone differs to our one
	if i.startswith("Date: "):
		date = i[6:-1]
		o.append("Date: %s\n" % improve_date(date))
		continue

	o.append(i)

for i in o:
	sys.stdout.write(i)
