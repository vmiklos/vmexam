import irssi, re, string, sys, urllib

"""interface to dict.sztaki.hu
type /py load sztakidict to load it
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
	raw = []

	if len(argv) > 1:
		lang = argv[0]
		word = argv[1]
	else:
		lang = "en"
		word = argv[0]

	if lang == "hu":
		url = "http://szotar.sztaki.hu/dict_search.php?S=W&L=HUN%3AENG%3AEngHunDict&W=" + word
	if lang == "hu2de":
		url = "http://szotar.sztaki.hu/dict_search.php?S=W&L=HUN%3AGER%3AGerHunDict&W=" + word
	else:
		url = "http://szotar.sztaki.hu/dict_search.php?S=W&W=" + word

	try:
		socket = urllib.urlopen(url)
	except IOError, str:
		print "problem: %s" % str
		return
	while True:
		line = socket.readline()
		if not line:
			break
		if line.find("nbsp") > 0:
			raw.append(re.sub(r'.*&nbsp;(.*)<br/>\n', r'\1', line))
	if len(raw):
		first = True
		for i in raw:
			if first:
				first = False
			else:
				sys.stdout.write(", ")
			sys.stdout.write(unicode(re.sub(r'\&\#([0-9]+);', rec, i), "latin2"))
		print
	else:
		print "not found"
	socket.close()

irssi.command_bind('dict', cmd_dict)
