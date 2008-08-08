import os, stat, re
from email.Utils import formatdate
from mod_python import apache
from xml.sax.saxutils import escape
from random import choice

quotepath = "/home/vmiklos/public_html/logs/irc"
quoteurl = "http://frugalware.org/~vmiklos/logs/irc"

class Quote:
	def __init__(self, dir, filename):
		self.filename = filename
		self.title = filename.replace('-', ' ')
		path = os.path.join(dir, filename)
		sock = open(path)
		self.content = "".join(sock.readlines())
		sock.close()
		self.time = os.stat(path)[stat.ST_MTIME]
	def compare(self, a, b):
		if a.time > b.time:
			return -1
		elif a.time == b.time:
			return 0
		else:
			return 1
	def getlist(req, all=False, random=False):
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
		if random:
			quotes = [choice(quotes)]
		elif not all:
			quotes = quotes[:10]
		return quotes
	getlist = staticmethod(getlist)


class Html:
	def __init__(self, req, title, link, desc):
		self.req = req
		self.title = title
		self.desc = desc
		self.link = link
		self.items = []
	def additem(self, title, link, desc, pubDate):
		self.items.append([title, link, desc, pubDate])
	def output(self):
		self.req.content_type = 'text/html'
		self.req.write("""
<?xml version="1.0" encoding="ISO-8859-2"?>
<html id="feedHandler">
  <head>
    <title>Viewing Feed</title>
    <link rel="stylesheet" href="../irc.css" type="text/css" media="all"/>
  </head>
  <body>
    <div id="feedHeaderContainer"><div id="feedHeader" dir="ltr"></div></div>

    <div id="feedBody">
      <div id="feedTitle">
        <div id="feedTitleContainer">
          <h1 id="feedTitleText">%s</h1>
          <h2 id="feedSubtitleText">%s</h2>
        </div>
      </div>
      <div id="feedContent">
""" % (self.title, self.desc))
		for title, link, desc, pubDate in self.items:
			self.req.write("""
<div class="entry"><h3><a href="%s">%s</a></h3><p class="feedEntryContent">
%s
</p></div>
""" % (link, title, desc))
		self.req.write("""
     A total of %d items has been printed.
     </div>
    </div>
  </body>
</html>
""" % len(self.items))
		return apache.OK

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

def handler(req):
	if "PATH_INFO" in req.subprocess_env.keys() and req.subprocess_env["PATH_INFO"] == "/all":
		html = Html(req, "~/vmiklos/rss/irc", quoteurl, "VMiklos' IRC Quotes")
		for i in Quote.getlist(req, all=True):
			html.additem(i.title, "%s/%s" % (quoteurl, i.filename), escape(i.content).replace("\n", "<br />"), formatdate(i.time, True))
		return html.output()
	elif "PATH_INFO" in req.subprocess_env.keys() and req.subprocess_env["PATH_INFO"] == "/random":
		html = Html(req, "~/vmiklos/rss/irc", quoteurl, "VMiklos' Random IRC Quotes")
		for i in Quote.getlist(req, random=True):
			html.additem(i.title, "%s/%s" % (quoteurl, i.filename), escape(i.content).replace("\n", "<br />"), formatdate(i.time, True))
		return html.output()
	else:
		rss = Rss(req, "~/vmiklos/rss/irc", quoteurl, "VMiklos' IRC Quotes RSS")
		for i in Quote.getlist(req):
			rss.additem(i.title, "%s/%s" % (quoteurl, i.filename), escape(escape(i.content).replace("\n", "<br />\n")), formatdate(i.time, True))
		return rss.output()
