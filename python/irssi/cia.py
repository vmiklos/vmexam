import irssi, urllib, threading, timeoutsocket
from sgmllib import SGMLParser

"""interface to cia.vc
type /py load cia to load it
usage:
	/cia"""

__author__ = "Miklos Vajna <vmiklos@frugalware.org>"
__version__ = "0.1"
__date__ = "Fri, 22 Jun 2007 00:20:02 +0200"
__copyright__ = "Copyright (c) 2007 Miklos Vajna"
__license__ = "GPL"

class HTMLParser(SGMLParser):
	def reset(self):
		SGMLParser.reset(self)
		self.inrow = False
		self.rows = []
		self.row = []

	def start_div(self, attrs):
		for k, v in attrs:
			if k == "class" and v == "row":
				self.inrow = True
	
	def start_td(self, attrs):
		for k, v in attrs:
			if k == "class" and v == "j":
				self.indesc = True

	def end_div(self):
		if self.inrow:
			self.rows.append("".join(self.row))
			self.row = []
			self.inrow = False

	def handle_data(self, text):
		if self.inrow:
			self.row.append(text)


def cmd_cia(data, server, witem):
	"""data - contains the parameters for /dict
server - the active server in window
witem - the active window item (eg. channel, query)
        or None if the window is empty"""
	timeoutsocket.setDefaultSocketTimeout(20)
	try:
		sock = urllib.urlopen("http://cia.vc/stats/author/Miklos%20Vajna%20%3Cvmiklos%40frugalware.org%3E")
		data = sock.read()
		sock.close()
	except timeoutsocket.Timeout, s:
		print s
		return
	except IOError, s:
		print s
		return

	parser = HTMLParser()
	parser.reset()
	parser.feed(data)
	parser.close()

	for i in parser.rows:
		if "so far today" in i:
			print i

irssi.command_bind('cia', cmd_cia)
