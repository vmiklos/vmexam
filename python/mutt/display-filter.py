#!/usr/bin/env python

import time, rfc822, sys, re
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
		if not tz or not tz[9]:
			return input
		return "%s (%s)" % (formatdate(time.mktime(tz[:9])-tz[9]-(time.timezone), True), input)

lines = sys.stdin.readlines()

ignore = False
ignorenext = False

o = []

for i in lines:
	if i.startswith("Date: "):
		date = i[6:-1]
		o.append("Date: %s\n" % improve_date(date))

	# gpg spam
	elif re.search("^gpg:.*aka", i):
		continue

	# spam from the sourceforge.net lists
	elif i.startswith("This SF.Net email is sponsored by"):
		ignore = True
		del o[-1]
	elif i.startswith("http://") and ignore:
		ignore = False

	# spam from freemail.hu
	elif i == "___________________________\n":
		ignorenext = True
	elif ignorenext:
		ignorenext = False

	elif not ignore:
		o.append(i)

for i in o:
	sys.stdout.write(i)
