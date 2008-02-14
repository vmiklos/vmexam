import irssi, re, string, sys, urllib, popen2

"""interface to aspell
type /py load spell to load it
type /spell word to use it
you can define the spelling language on a server and on a channel basis. the
second takes precedence
"""

__author__ = "Miklos Vajna <vmiklos@frugalware.org>"
__version__ = "0.1"
__date__ = "Thu, 21 Jun 2007 13:51:23 +0200"
__copyright__ = "Copyright (c) 2007 Miklos Vajna"
__license__ = "GPL"

server_langs = {
		'freenode': 'en'
		}
channel_langs = {
		'#frugalware.hu': 'hu'
		}

aliases = {
		'hu': 'hu-HU',
		'en': 'en_US'
		}

def cmd_spell(data, server, witem):
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
		word = argv[1]
	else:
		lang = "en"
		if server.tag in server_langs.keys():
			lang = server_langs[server.tag]
		if witem:
			if witem.name in channel_langs.keys():
				lang = channel_langs[witem.name]
		word = argv[0]
	if lang in aliases.keys():
		lang = aliases[lang]
	
	pout, pin = popen2.popen2('hunspell -a -d %s' % lang)
	pin.write("%s\n" % word)
	pin.close()
	ret = pout.readlines()
	pout.close()
	if ret[1].startswith("&"):
		print unicode(ret[1].split(":")[1].strip(), "latin2")
	else:
		print "OK"

irssi.command_bind('spell', cmd_spell)
