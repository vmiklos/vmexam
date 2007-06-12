#!/usr/bin/env python

# sztaki szotar cmdline interface
# don't use it or i'll be killed ;-)

import re, string, sys, urllib

def rec(match):
	return(chr(string.atoi(match.group()[2:-1])))

raw = []

if len(sys.argv) > 2:
	lang = sys.argv[1]
	word = sys.argv[2]
else:
	lang = "en"
	word = sys.argv[1]

if lang == "hu":
	url = "http://szotar.sztaki.hu/dict_search.php?S=W&L=HUN%3AENG%3AEngHunDict&W=" + word
else:
	url = "http://szotar.sztaki.hu/dict_search.php?S=W&W=" + word

try:
	socket = urllib.urlopen(url)
except IOError, str:
	print "problem: %s" % str
	sys.exit()
while True:
	line = socket.readline()
	if not line:
		break
	if line.find("nbsp") > 0:
		raw.append(re.sub(r'.*&nbsp;(.*)<br/>\n', r'\1', line))
if len(raw):
	first = True
	for i in raw:
		if first:
			first = False
		else:
			sys.stdout.write(", ")
		sys.stdout.write(re.sub(r'\&\#([0-9]+);', rec, i))
	print
else:
	print "not found"
socket.close()
