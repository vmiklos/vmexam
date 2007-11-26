#!/usr/bin/env python

import irssi, time, os

def timer():
	sock = open(os.path.join(os.environ['HOME'], ".irssi", "chanstat"), "a")
	win = irssi.active_win()
	if irssi.active_win().name:
		name = irssi.active_win().name
		tag = "n/a"
	else:
		name = irssi.active_win().active.name
		if win.active_server:
			tag = irssi.active_win().active_server.tag
		else:
			tag = "n/a"
	sock.write("%s %s %s\n" % (int(time.time()), tag, name))
	sock.close()
	irssi.timeout_add(1000*60, timer)

timer()
