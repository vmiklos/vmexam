"""
	prefix each message

	type /py load prefix to load it

	changelog:
	- 0.2: rewritten in python
	- 0.1: initial perl script by Gabor Adam TOTH
"""

__author__ = "Miklos Vajna <vmiklos@frugalware.org>"
__version__ = "0.2"
__date__ = "Fri, 22 Jun 2007 00:44:12 +0200"
__copyright__ = "Copyright (c) 2007 Miklos Vajna"
__license__ = "GPL"

import irssi, re

def send(msg, server, witem):
	prefix = irssi.settings_get_str("prefix")
	targets = irssi.settings_get_str("prefix_targets")
	cmdchars = irssi.settings_get_str("cmdchars")
	if re.match("^([%s]|%s)" % (cmdchars, prefix), msg):
		return
	if not check_target(witem.name, targets):
		return
	signal = irssi.signal_get_emitted()
	msg = "%s%s" % (prefix, msg)
	irssi.signal_stop()
	irssi.signal_emit(signal, msg, server, witem)

def check_target(target, targets):
	target = target.lower()
	targets = targets.lower().split(' ')
	if target in targets or targets[0] == "*":
		return True

irssi.settings_add_str(__name__, 'prefix', '')
irssi.settings_add_str(__name__, 'prefix_targets', '*')
irssi.signal_add("send command", send)
