import os, sys
sys.path.append(os.path.join(os.environ['HOME'], "git", "mxw"))
import irssi, re, string, sztakidict
from accent import unaccent

"""interface to dict.sztaki.hu
type /py load dict to load it
usage:
	/dict lang word
or
	/dict word (defaults to en)"""

__author__ = "Miklos Vajna <vmiklos@frugalware.org>"
__version__ = "0.1"
__date__ = "Thu, 21 Jun 2007 03:28:51 +0200"
__copyright__ = "Copyright (c) 2007 Miklos Vajna"
__license__ = "GPL"

def rec(match):
	return(chr(string.atoi(match.group()[2:-1])))

def cmd_dict(data, server, witem):
	"""data - contains the parameters for /dict
server - the active server in window
witem - the active window item (eg. channel, query)
        or None if the window is empty"""
	argv = data.split(' ')
	tmp = []
	for i in argv:
		if len(i):
			tmp.append(i)
	argv = tmp
	del tmp

	if len(argv) > 1:
		lang = argv[0]
		word = " ".join(argv[1:])
	else:
		lang = "en"
		word = " ".join(argv)

	try:
		ret = sztakidict.helper(lang, word)
	except IOError, str:
		print "problem: %s" % str
		return
	print unicode(ret, "utf8").encode("latin2")

irssi.command_bind('dict', cmd_dict)
