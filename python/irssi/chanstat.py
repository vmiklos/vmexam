#!/usr/bin/env python

import irssi, time, os

def timer():
	win = irssi.active_win()
	if win.active and win.active_server:
		sock = open(os.path.join(os.environ['HOME'], ".irssi", "chanstat"), "a")
		sock.write("%s %s %s\n" % (int(time.time()), win.active_server.tag, win.active.name))
		sock.close()
	irssi.timeout_add(1000*60, timer)

timer()
