from mod_python import apache
import sys, os
sys.path.append("/usr/lib")
import feedparser, time, pickle

feeds = None
dumpfile = "/home/vmiklos/ftp/vmiklos.hu/htdocs/startup/feeds"

def dumpcache():
	global feeds
	try:
		socket = open(dumpfile, "w")
		pickle.dump(feeds, socket)
		socket.close()
	except TypeError:
		os.remove(dumpfile)

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
	try:
		ret.append('<div id="right" class="sideboxpadding">')
		ret.append('<div class="boxheader">%s<br /></div>' % (feed.feed.title.encode('ascii', 'xmlcharrefreplace')))
		ret.append('<div class="sidecontent"><ul>')
		for i in feed.entries:
			ret.append('<li><a href="%s">%s</a></li>' % (i.link, i.title.encode('ascii', 'xmlcharrefreplace')))
		ret.append('</ul></div></div>')
		ret.append('<div id="right" class="dummybox">')
		ret.append('</div>')
	except AttributeError:
		feed.feed = None
		ret = []
	return "\n".join(ret)

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
	<a href="http://vmiklos.hu/blog">blog</a> |
	<a href="http://blogs.frugalware.org/vmiklos">fwblog</a> |
	<a href="http://bugs.frugalware.org/?dev=vmiklos&order=lastedit&sort=desc">my tasks</a> |
	<a href="http://bugs.frugalware.org/?string=[SEC]&order=lastedit&sort=desc">sec tasks</a> |
	<a href="http://bugs.frugalware.org/?do=roadmap">roadmap</a> |
	<a href="http://bugs.frugalware.org/?sev[]=5">critical bugs</a> |
	<a href="http://rss.gmane.org/gmane.linux.frugalware.scm">-git rss</a> |
	<a href="http://ftp.frugalware.org/pub/other/people/vmiklos/">devspace</a> |
	<a href="http://vmiklos.hu">homepage</a>
	</div>
	<div class="dummybox"></div>
	<div id="main">
	""")
	out.append(dumpfeed("http://rss.slashdot.org/Slashdot/slashdot"))
	out.append(dumpfeed("http://frugalware.org/~vmiklos/rss/irc/irc.py"))
	out.append(dumpfeed("http://qdb.hu/latest/rss"))
	out.append(dumpfeed("http://bash.hu/rss"))
	out.append(dumpfeed("http://magerquark.com/feeds/bash.org/Latest.ashx"))
	out.append(dumpfeed("http://frugalware.org/~vmiklos/rss/legalja/legalja.py"))
	out.append("""
	</div>
	</body>
	</html>
	""")
	req.write("".join(out))
	dumpcache()
	return apache.OK
