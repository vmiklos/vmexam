#!/usr/bin/env python3

import time, sys, re
import email.utils

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

lines = sys.stdin.readlines()

ignorenext = False
ignoresf = False
ym = None
msgid = None

o = []

for i in lines:
	# add local date _as well_ if the timezone differs to our one
	if i.startswith("Date: "):
		date = i[6:-1]
		try:
			ym = time.strftime("%Y-%m", email.utils.parsedate_tz(date)[:-1])
		except TypeError:
			pass
		o.append("Date: %s\n" % improve_date(date))
		continue

	# gpg spam
	elif re.search("^gpg:.*aka", i):
		continue

	# sch archive permalink
	elif i.startswith("Message-id: "):
		msgid = i[13:-2]
	elif i.startswith("List-Archive: <https://lists.sch.bme.hu/wws/arc/"):
		l = i.split('/')[-1].split('>')[0]
		if ym:
			o.append("X-Sch-Url: https://lists.sch.bme.hu/wws/arcsearch_id/%s/%s/%s\n" % (l, ym, msgid))
			continue
	elif i.startswith("List-Archive: "):
		continue

	o.append(i)

for i in o:
	sys.stdout.write(i)
