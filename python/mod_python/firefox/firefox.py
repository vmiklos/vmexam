from mod_python import apache
import sys
sys.path.append("/usr/lib")
import feedparser, time, pickle

feeds = None
dumpfile = "/home/vmiklos/public_html/firefox/feeds"

def dumpcache():
	global feeds
	socket = open(dumpfile, "w")
	pickle.dump(feeds, socket)
	socket.close()

def fetchfeed(url):
	global feeds
	feed = feedparser.parse(url)
	feed.updated = time.time()
	feeds[url] = feed
	return feed

def dumpfeed(url):
	global feeds
	ret = []
	if not feeds:
		try:
			socket = open(dumpfile, "r")
			feeds = pickle.load(socket)
			socket.close()
		except IOError:
			feeds = {}
	if url in feeds.keys():
		feed = feeds[url]
		# check if the feed is outdated
		if (time.time() - feed.updated) > 1800:
			feed = fetchfeed(url)
	else:
		feed = fetchfeed(url)
	ret.append('<div id="right" class="sideboxpadding">')
	ret.append('<div class="boxheader">%s<br /></div>' % (feed.feed.title.encode('ascii', 'xmlcharrefreplace')))
	ret.append('<div class="sidecontent">')
	for i in feed.entries:
		ret.append('<a href="%s">%s</a><br />' % (i.link, i.title.encode('ascii', 'xmlcharrefreplace')))
	ret.append('</div></div>')
	ret.append('<div id="right" class="dummybox">')
	ret.append('</div>')
	return "\n".join(ret)

if __name__ == "firefox":
	def handler(req):
		out = []

		req.content_type = "text/html; charset=utf-8"
		out.append("""
		<html>
		<head>
		<link rel="stylesheet" type="text/css" href="firefox.css" />
		<title>Firefox Startup Page</title>
		</head>
		<body>
		<div class="header">
		<a href="http://frugalware.org/">frugalware</a> |
		<a href="http://openblog.hu/vmiklos">blog</a> |
		<a href="http://bugs.frugalware.org/?dev=vmiklos">my tasks</a> |
		<a href="http://bugs.frugalware.org/?string=[SEC]">sec tasks</a>
		</div>
		<div class="dummybox"></div>
		<div id="main">
		""")
		out.append(dumpfeed("http://feeds.feedburner.com/HUP"))
		out.append(dumpfeed("http://frugalware.org/~vmiklos/rss/irc/irc.py"))
		out.append(dumpfeed("http://rss.slashdot.org/Slashdot/slashdot"))
		out.append("""
		</div>
		</body>
		</html>
		""")
		req.write("".join(out))
		dumpcache()
		return apache.OK
