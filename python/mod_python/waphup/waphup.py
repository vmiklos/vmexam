from mod_python import apache
import feedparser

import sys
sys = reload(sys)
sys.setdefaultencoding("utf-8")

def display_nodes(url, req):
	feed = feedparser.parse(url)
	req.write("""<?xml version="1.0" encoding="utf-8"?>
	<!DOCTYPE wml PUBLIC "-//WAPFORUM//DTD WML 1.1//EN"
	"http://www.wapforum.org/DTD/wml_1.1.xml">

	<wml>

	<card id="XML" title="waphup">
	<p>
	""")
	for i in feed.entries:
		req.write('<a href="?n=%s">%s</a><br />' % (i.link, i.title))
	req.write("""
	</p>
	</card>

	</wml>
	""")

def display_node(url, req):
	def getnode(xml):
		for i in xml.getElementsByTagName("div"):
			try:
				if i.attributes['id'].value.startswith("node-") and i.attributes['class'].value == "node":
					return i.getElementsByTagName("div")[0]
			except KeyError:
				pass
	req.write("""<?xml version="1.0" encoding="utf-8"?>
	<!DOCTYPE wml PUBLIC "-//WAPFORUM//DTD WML 1.1//EN"
	"http://www.wapforum.org/DTD/wml_1.1.xml">

	<wml>

	<card id="XML" title="waphup">
	<p>
	""")
	import urllib, tidy
	from xml.dom import minidom
	sock = urllib.urlopen(url)
	buf = sock.read()
	sock.close()
	options = dict(output_xhtml=1, add_xml_decl=1, input_encoding="utf8", output_encoding="utf8")
	node = tidy.parseString(buf[:20000], **options)
	
	try:
		xml = minidom.parseString(node.__str__())
		node = getnode(xml).toxml()
		req.write(node)
	except Exception, s:
		req.write("Error while parsing the node: %s" % s)
	req.write("""
	</p>
	</card>

	</wml>
	""")

def handler(req):
	req.content_type = 'text/vnd.wap.wml; charset=utf-8'

	qs = req.subprocess_env['QUERY_STRING'];
	if not qs.startswith("n=http://feeds.feedburner.com/~r/HUP/"):
		display_nodes("http://feeds.feedburner.com/HUP", req)
	else:
		display_node(qs[2:], req)
	return apache.OK
