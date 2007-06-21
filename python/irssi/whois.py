import irssi, urllib, socket

"""interface to completewhois.com
type /py load whois to load it
usage:
	/pywhois hostname
or
	/pywhois ip

it will tell you the first description line (usually the name of the network
provider) and the country code (useful when no reverse dns is available)"""

__author__ = "Miklos Vajna <vmiklos@frugalware.org>"
__version__ = "0.1"
__date__ = "Thu, 21 Jun 2007 16:00:38 +0200"
__copyright__ = "Copyright (c) 2007 Miklos Vajna"
__license__ = "GPL"

def cmd_whois(data, server, witem):
	"""data - contains the parameters for /dict
server - the active server in window
witem - the active window item (eg. channel, query)
        or None if the window is empty"""

	argv = data.split(' ')

	# get the ip if this is a hostname
	ip = argv[0]

	try:
		socket.inet_aton(ip)
	except socket.error:
		ip = socket.gethostbyname(ip)

	sock = urllib.urlopen("http://www.completewhois.com/cgi2/whois.cgi?query=%s" % ip)
	descr = None
	country = None
	for i in sock.readlines():
		if i.startswith("descr: "):
			if not descr:
				descr = i.split(":")[1].strip()
		if i.startswith("country: "):
			if not country:
				country = i.split(":")[1].strip()
	sock.close()
	print "%s, %s" % (descr, country)

irssi.command_bind('pywhois', cmd_whois)
