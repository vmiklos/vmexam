import irssi, urllib
from xml.dom import minidom

"""interface to last.fm
type /py load lastfm to use it"""

__author__ = "Miklos Vajna <vmiklos@frugalware.org>"
__version__ = "0.1"
__date__ = "Thu, 21 Jun 2007 04:42:58 +0200"
__copyright__ = "Copyright (c) 2007 Miklos Vajna"
__license__ = "GPL"

def rec(match):
	return(chr(string.atoi(match.group()[2:-1])))

def cmd_lastfm(data, server, witem):
	"""data - contains the parameters for /dict
server - the active server in window
witem - the active window item (eg. channel, query)
        or None if the window is empty"""
	sock = urllib.urlopen("http://ws.audioscrobbler.com/1.0/user/EDITME/recenttracks.xml")
	xmldoc = minidom.parseString(sock.read())
	artist = xmldoc.getElementsByTagName('artist')[0].firstChild.toxml().encode("latin2")
	name = xmldoc.getElementsByTagName('name')[0].firstChild.toxml().encode("latin2")
	witem.command("/action %s is now playing %s - %s" % (witem.name, artist, name))

irssi.command_bind('lastfm', cmd_lastfm)
