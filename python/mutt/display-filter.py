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
ignoresf = False
ym = None
msgid = None

o = []

for i in lines:
	# add local date _as well_ if the timezone differs to our one
	if i.startswith("Date: "):
		date = i[6:-1]
		ym = time.strftime("%Y-%m", rfc822.parsedate_tz(date)[:-1])
		o.append("Date: %s\n" % improve_date(date))
		continue

	# gpg spam
	elif re.search("^gpg:.*aka", i):
		continue

	# spam from the sourceforge.net lists (commits lists)
	elif i.startswith("This was sent by the SourceForge.net"):
		ignore = True
		del o[-1]
	elif i.startswith("http://") and ignore:
		ignore = False
	# next spam from sf.net (normal lists)
	elif i == "------------------------------------------------------------------------------\n":
		ignoresf = True
		continue
	elif ignoresf and "http://p.sf.net/sfu/" in i:
		ignoresf = False
		continue
	elif ignoresf:
		continue
	# spam from freemail.hu
	elif i == "<!-- PATH STAT NUMBER ERROR -->\n":
		continue

	# sch archive permalink
	if i.startswith("Message-id: "):
		msgid = i[13:-2]
	if i.startswith("List-Archive: <https://lists.sch.bme.hu/wws/arc/"):
		l = i.split('/')[-1].split('>')[0]
		o.append("X-Sch-Url: https://lists.sch.bme.hu/wws/arcsearch_id/%s/%s/%s\n" % (l, ym, msgid))
	elif i.startswith("List-Archive: "):
		continue

	elif not ignore:
		o.append(i)

for i in o:
	sys.stdout.write(i)
