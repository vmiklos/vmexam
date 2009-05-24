from sgmllib import SGMLParser
import urllib
import time
from email.Utils import formatdate
from mod_python import apache

class BaseHTMLProcessor(SGMLParser):
	def reset(self):
		self.rss = None
		self.data = []
		self.intable = False

		self.title = None
		self.date = None
		self.body = None

		SGMLParser.reset(self)

	def handle_data(self, text):
		self.data.append(text.strip())

	def start_span(self, attrs):
		for k, v in attrs:
			if k == "class":
				self.data = []

	def end_span(self):
		text = "".join(self.data)
		if text == "Latest News":
			self.intable = True
		elif self.intable:
			if not self.title:
				self.title = text
			elif not self.date:
				try:
					self.date = formatdate(time.mktime(time.strptime(text, "%d.%m.%Y %H:%M:%S")), True)
				except ValueError:
					self.title = None
			elif not self.body:
				self.body = text
				self.rss.additem(self.title, self.date, self.body)
				self.title = self.date = self.body = None

class Rss:
	def __init__(self, req, title, link, desc):
		self.req = req
		self.title = title
		self.desc = desc
		self.link = link
		self.items = []
	def additem(self, title, pubDate, body):
		self.items.append([title, pubDate, body])
	def output(self):
		self.req.content_type = 'application/xml'
		self.req.write("""<?xml version="1.0" encoding="utf-8"?>
<rss version="2.0">
<channel>
<title>%s</title>
<description>%s</description>
<link>%s</link>\n""" % (self.title, self.desc, self.link))
		for title, pubDate, body in self.items:
			self.req.write("""\t<item>
		<title>%s</title>
		<description>%s</description>
		<pubDate>%s</pubDate>
	</item>\n""" % (title, body, pubDate))
		self.req.write("</channel>\n</rss>")
		return apache.OK

def handler(req):
	sock = urllib.urlopen("http://www.scootertechno.com/")
	parser = BaseHTMLProcessor()
	parser.rss = Rss(req, "Scooter RSS", "http://www.scootertechno.com/", "Official site, unofficial RSS.")
	parser.feed(sock.read())
	parser.close()
	sock.close()
	return parser.rss.output()
