#!/usr/bin/env python
# -*- coding: iso-8859-2 -*-

# enforces a ban on a given channel if the unbanner matches regex

import irssi, re

def mode(server, channel, who, host):
	if channel.split(' ')[0] == "#owl" and re.match(r"^\[.*\]$", who.split(' ')[0]) and re.match(r"-b", channel.split(' ')[1]):
		server.command("MODE %s +b %s" % (channel.split(' ')[0], channel.split(' ')[2]))

irssi.signal_add("event mode", mode)
