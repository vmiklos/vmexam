#!/usr/bin/env python

"""Counts the total elapsed time in an irssi pivate log.
The logs must have the $nick_%Y%m%d.log format

There is only one required parameter: the nick.
"""

__author__ = "Miklos Vajna <vmiklos@frugalware.org>"
__version__ = "0.1"
__date__ = "Fri, 29 Dec 2006 01:25:21 +0100"
__copyright__ = "Copyright (c) 2006 Miklos Vajna"
__license__ = "GPL"

import time, re, os, sys

startstr = "--- Log opened"
endstr = "--- Log closed"

def countlog(log):
	ts = None
	sum = 0

	sock = open(log)
	while True:
		line = sock.readline().strip()
		if not line:
			break
		elif line[:len(startstr)] == startstr:
			ts = time.strptime(line[len(startstr)+1:], "%a %b %d %H:%M:%S %Y")
		elif line[:len(endstr)] == endstr:
			if ts:
				sum += time.mktime(time.strptime(line[len(startstr)+1:],
					"%a %b %d %H:%M:%S %Y")) - time.mktime(ts)
				ts = None
	sock.close()
	return sum

total = 0
for root, dirs, files in os.walk("."):
	for file in files:
		if re.sub(r'_[0-9]{8}.log', '', file) == sys.argv[1]:
			total += countlog(file)
tv = time.gmtime(total)
print "%d year(s), %d month(s), %d day(s), %d hour(s), %d minute(s) and %d second(s)" % (tv[0]-1970,
		tv[1]-1, tv[2]-1, tv[3], tv[4], tv[5])
