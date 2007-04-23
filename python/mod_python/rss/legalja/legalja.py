from sgmllib import SGMLParser
#import htmlentitydefs
import urllib
import time
from email.Utils import formatdate
from mod_python import apache

class BaseHTMLProcessor(SGMLParser):
	def reset(self):
		self.waitfortitle = False
		self.tmplink = None
		self.tmptime = None
		self.rss = None
		SGMLParser.reset(self)
	def handle_data(self, text):
		if self.waitfortitle:
			self.rss.additem(text.strip(), self.tmplink, self.tmptime)
			self.waitfortitle = False
	def start_td(self, attrs):
		for k, v in attrs:
			if k == "class" and v == "text_1":
				self.waitfortitle = True
	def start_a(self, attrs):
		for k, v in attrs:
			if k == "href" and v[:9] == "?leg=view":
				# title, link, pubdate
				self.tmplink = "http://www.legalja.hu/" + v.replace("&", "&amp;")
				self.tmptime = formatdate(time.mktime(time.strptime(v[14:22], "%Y%m%d")), True)

class Rss:
	def __init__(self, req, title, link, desc):
		self.req = req
		self.title = title
		self.desc = desc
		self.link = link
		self.items = []
	def additem(self, title, link, pubDate):
		self.items.append([title, link, pubDate])
	def output(self):
		self.req.content_type = 'application/xml'
		self.req.write("""<?xml version="1.0" encoding="iso-8859-2"?>
<rss version="2.0">
<channel>
<title>%s</title>
<description>%s</description>
<link>%s</link>\n""" % (self.title, self.desc, self.link))
		for title, link, pubDate in self.items:
			self.req.write("""<item>
			<title>%s</title>
			<link>%s</link>
			<pubDate>%s</pubDate>
			</item>\n""" % (title, link, pubDate))
		self.req.write("</channel>\n</rss>")
		return apache.OK

def handler(req):
	sock = urllib.urlopen("http://www.legalja.hu/")
	parser = BaseHTMLProcessor()
	parser.rss = Rss(req, "Legalja.hu RSS", "http://www.legalja.hu", "Piros feher zold")
	parser.feed(sock.read())
	parser.close()
	sock.close()
	return parser.rss.output()
