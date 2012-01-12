#!/usr/bin/env python
# -*- coding: iso-8859-2 -*-

"""
	strip latin2 accents from each message

	type /py load accent to load it

	changelog:
	- 2.1: added /nounaccent command
	- 2.0: rewritten in python
	- 1.34: initial perl script by Tamas SZERB
"""

__author__ = "Miklos Vajna <vmiklos@frugalware.org>"
__version__ = "2.1"
__date__ = "Tue, 26 Jun 2007 16:11:21 +0200"
__copyright__ = "Copyright (c) 2007 Miklos Vajna"
__license__ = "GPL"

import irssi

stripped_out = False

def unaccent(s):
	ret = []
	fro = "??????????????????"
	to = "AEIOOOUUUaeiooouuu"
	for i in s:
		if i in fro:
			ret.append(to[fro.index(i)])
		else:
			ret.append(i)
	return "".join(ret)

def send(msg, server, witem):
	global stripped_out

	if stripped_out:
		return
	signal = irssi.signal_get_emitted()
	if msg.startswith("/nounaccent"):
		msg = msg[len("/nounaccent")+1:]
	elif not msg.startswith("/dict") and not msg.startswith("/spell"):
		msg = unaccent(msg)
	stripped_out = True
	irssi.signal_stop()
	irssi.signal_emit(signal, msg, server, witem)
	stripped_out = False

def nounaccent(msg, server, witem):
	irssi.signal_emit("send command", "/ /nounaccent" + msg, server, witem)

irssi.signal_add("send command", send)
irssi.command_bind('nounaccent', nounaccent)
