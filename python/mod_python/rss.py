from mod_python import apache

class Rss:
	def __init__(self, req, title, link, desc):
		self.req = req
		self.title = title
		self.desc = desc
		self.link = link
		self.items = []
	def additem(self, title, link, desc=None, author=None, pubDate=None):
		self.items.append([title, link, desc, author, pubDate])
	def output(self):
		self.req.content_type = 'application/xml'
		self.req.write("""<?xml version="1.0" encoding="utf-8"?>
<rss version="2.0">
<channel>
<title>%s</title>
<description>%s</description>
<link>%s</link>\n""" % (self.title, self.desc, self.link))
		for title, link, desc, author, pubDate in self.items:
			self.req.write("""<item>
	<title>%s</title>
	<link>%s</link>\n""" % (title, link))
			if desc:
				self.req.write("\t<description>%s</description>\n" % desc)
			if author:
				self.req.write("\t<author>%s</author>\n" % author)
			if pubDate:
				self.req.write("\t<pubDate>%s</pubDate>\n" % pubDate)
			self.req.write("</item>\n")
		self.req.write("</channel>\n</rss>")
		return apache.OK

if __name__ == "rss":
	def handler(req):
		rss = Rss(req, "title", "http://foo.com", "desc")
		rss.additem("ititle", "http://bar.com", "idesc")
		rss.additem("ititle2", "http://baz.com", "idesc2")
		return rss.output()
