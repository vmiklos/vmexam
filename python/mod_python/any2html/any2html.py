from mod_python import apache
import urllib

def usage(req):
	req.write("Usage: http://frugalware.org%s/http://foo.com/bar.xls" %
			"/".join(req.subprocess_env["SCRIPT_NAME"].split("/")[:-1]))

if __name__ == "any2html":
	def handler(req):
		req.content_type = "text/html"
		try:
			url = req.subprocess_env["PATH_INFO"].strip("/").replace(":/", "://")
			if url == "":
				raise
			#url = "http://wap.google.com/gwt/n?output=html&u=%s" % url
			url = "http://wap.google.com/gwt/n?mrestrict=xhtml&u=%s" % url
			sock = urllib.urlopen(url)
			req.write(sock.read())
			sock.close()
			return apache.OK
		except:
			usage(req)
			return apache.OK
