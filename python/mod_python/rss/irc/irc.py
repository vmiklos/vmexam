import os, stat, re
from email.Utils import formatdate
from mod_python import apache
from xml.sax.saxutils import escape

quotepath = "/home/vmiklos/public_html/logs/irc"
quoteurl = "http://frugalware.org/~vmiklos/logs/irc"

class Quote:
	def __init__(self, dir, filename):
		self.filename = filename
		self.title = filename.replace('-', ' ')
		path = os.path.join(dir, filename)
		sock = open(path)
		self.content = "<br />".join(sock.readlines())
		sock.close()
		self.time = os.stat(path)[stat.ST_MTIME]
	def compare(self, a, b):
		if a.time > b.time:
			return -1
		elif a.time == b.time:
			return 0
		else:
			return 1

class Rss:
	def __init__(self, req, title, link, desc):
		self.req = req
		self.title = title
		self.desc = desc
		self.link = link
		self.items = []
	def additem(self, title, link, desc, pubDate):
		self.items.append([title, link, desc, pubDate])
	def output(self):
		self.req.content_type = 'application/xml'
		self.req.write("""<?xml version="1.0" encoding="iso-8859-2"?>
<rss version="2.0">
<channel>
<title>%s</title>
<description>%s</description>
<link>%s</link>\n""" % (self.title, self.desc, self.link))
		for title, link, desc, pubDate in self.items:
			self.req.write("""<item>
			<title>%s</title>
			<link>%s</link>
			<description>%s</description>
			<pubDate>%s</pubDate>
			</item>\n""" % (title, link, desc, pubDate))
		self.req.write("</channel>\n</rss>")
		return apache.OK

def getquotes():
	ignores = []
	quotes = []
	sock = open(os.path.join(quotepath, ".htaccess"))
	for i in sock.readlines():
		if i.startswith("IndexIgnore"):
			ignores.append(i.strip("IndexIgnore ").replace("*", ".*").strip())
	sock.close

	for root, dirs, files in os.walk(quotepath):
		for file in files:
			if not file.startswith("."):
				ignore = False
				for i in ignores:
					if re.match(i, file):
						ignore = True
						break
				if not ignore:
					quotes.append(Quote(root, file))

	quotes.sort(quotes[0].compare)
	quotes = quotes[:10]
	return quotes

def handler(req):
	quotes = getquotes()
	rss = Rss(req, "~/vmiklos/rss/irc", quoteurl, "VMiklos' IRC Quotes RSS")
	for i in getquotes():
		rss.additem(i.title, "%s/%s" % (quoteurl, i.filename), escape(i.content), formatdate(i.time, True))
	return rss.output()
