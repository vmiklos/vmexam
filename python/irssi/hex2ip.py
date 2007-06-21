import irssi, socket

"""hex2ip converter
type /py load hex2ip to load it
usage:
	/hex2ip aabbccdd"""

__author__ = "Miklos Vajna <vmiklos@frugalware.org>"
__version__ = "0.1"
__date__ = "Thu, 21 Jun 2007 14:00:00 +0200"
__copyright__ = "Copyright (c) 2007 Miklos Vajna"
__license__ = "GPL"

def hex2ip(s):
	return ".".join(["%d"%int(n, 16) for n in (s[0:2],s[2:4],s[4:6],s[6:8])])

def cmd_hex2ip(data, server, witem):
	"""data - contains the parameters for /dict
server - the active server in window
witem - the active window item (eg. channel, query)
        or None if the window is empty"""
	argv = data.split(' ')
	try:
		host = socket.gethostbyaddr(hex2ip(argv[0]))
	except socket.error, str:
		print str[1]
		return
	print host[0]

irssi.command_bind('hex2ip', cmd_hex2ip)
