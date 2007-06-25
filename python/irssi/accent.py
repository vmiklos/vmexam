#!/usr/bin/env python
# -*- coding: iso-8859-2 -*-

"""
	strip accents from each message

	type /py load accent to load it

	changelog:
	- 2.0: rewritten in python
	- 1.34: initial perl script by Tamas SZERB
"""

__author__ = "Miklos Vajna <vmiklos@frugalware.org>"
__version__ = "2.0"
__date__ = "Tue, 26 Jun 2007 00:08:48 +0200"
__copyright__ = "Copyright (c) 2007 Miklos Vajna"
__license__ = "GPL"

import irssi

stripped_out = False

def unaccent(s):
	ret = []
	fro = "ÁÉÍÓÖŐÚÜŰáéíóöőúüű"
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
	msg = unaccent(msg)
	stripped_out = True
	irssi.signal_stop()
	irssi.signal_emit(signal, msg, server, witem)
	stripped_out = False

irssi.signal_add("send command", send)
