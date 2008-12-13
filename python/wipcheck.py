#!/usr/bin/env python

import os, sys

# a simple script to check if there are outdated pkgs in a wip repo
# usage: wipcheck.py bmf frugalware-current

def vercmp(a, b):
	sock = os.popen("vercmp %s %s" % (a, b))
	buf = sock.readline()
	sock.close()
	return int(buf)

try:
	wip = sys.argv[1]
	current = sys.argv[2]
except IndexError:
	print "usage: ./wipcheck.py bmf frugalware-current"
	sys.exit(1)

sock = os.popen("pacman-g2 -Sl")
buf = sock.readlines()
sock.close()

wpkgs = {}
cpkgs = {}

for i in buf:
	if i.startswith(wip + " "):
		l = i.split(' ')
		wpkgs[l[1]] = l[2].strip()
	if i.startswith(current + " "):
		l = i.split(' ')
		cpkgs[l[1]] = l[2].strip()

for k, v in wpkgs.items():
	if k in cpkgs.keys() and vercmp(v, cpkgs[k]) < 0:
		print "%s-%s is in %s, but %s is in %s, that's bad" % (k, v, wip, cpkgs[k], current)
