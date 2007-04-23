import urllib, sys

sock = urllib.urlopen("http://www.completewhois.com/cgi2/whois.cgi?query=%s" % sys.argv[1])
for i in sock.readlines():
	if i.startswith("descr: "):
		print i
		break
