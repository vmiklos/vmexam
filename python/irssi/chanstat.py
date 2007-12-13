#!/usr/bin/env python

import irssi, time, os

ignore = []

statfile = os.path.join(os.environ['HOME'], ".irssi", "chanstat")

def cmd_chanstat(data, server, witem):
	def chancmp(a, b):
		return cmp(a[1], b[1])

	prevdate = None
	datelimit = None
	datestr = ""
	if data == "yesterday":
		datestr = " previous"
		prevdate = 86400
		datelimit = 86400

	nick = irssi.active_server().nick

	current = time.localtime()
	fro = int(time.mktime(current) - current.tm_hour*3600 - current.tm_min*60 - current.tm_sec)
	if prevdate:
		fro -= prevdate
	to = None
	if datelimit:
		to = fro + datelimit

	sock = open(statfile)

	lines = []
	for i in sock.readlines():
		items = i.strip().split(' ')
		if int(items[0]) > fro and items[2] not in ignore and items[2][0] == "#":
			if to:
				if int(items[0]) < to:
					lines.append(items)
			else:
				lines.append(items)
	sock.close()
	total = len(lines)

	chans = {}

	for i in lines:
		chan = i[2]
		if chan not in chans.keys():
			chans[chan] = 1
		else:
			chans[chan] += 1

	sorted = []
	for k, v in chans.items():
		sorted.append([k, v])

	sorted.sort(chancmp, reverse=True)

	if nick[-1] == 's':
		s = "'"
	else:
		s = "'s"

	dstr = "%s%s day: %s" % (s, datestr, " ".join(["%s [%sm]" % (i, j) for i, j in sorted]))
	if witem:
		witem.command("/me %s" % dstr)
	else:
		print "* %s %s" % (nick, dstr)

def timer():
	win = irssi.active_win()
	if win.active and win.active_server:
		sock = open(statfile, "a")
		sock.write("%s%s %s\n" % (int(time.time()), win.active_server.tag, win.active.name))
		sock.close()
	irssi.timeout_add(1000*60, timer)

timer()

irssi.command_bind('chanstat', cmd_chanstat)
