#!/usr/bin/env python

__author__ = "Miklos Vajna <vmiklos@frugalware.org>"
__version__ = "0.1"
__date__ = "Tue, 21 Nov 2006 22:54:11 +0100"
__copyright__ = "Copyright (c) 2006 Miklos Vajna"
__license__ = "GPL"

from mod_python import apache
from email.Utils import formatdate
import pickle, telnetlib, time, os, sys, re

def ping(host, timeout = 1):
	sock = os.popen("ping -c 5 -W %d %s" % (timeout, host))
	for i in sock.readlines():
		if re.match(".*packets transmitted", i):
			percent = re.sub(r'.*received, (.*)% packet.*', r'\1', i.strip())
			break
	if int(percent) > 40:
		return False
	else:
		return True

class Rss:
	def __init__(self, req, title, link, desc):
		self.req = req
		self.title = title
		self.desc = desc
		self.link = link
		self.items = []
	def additem(self, title, author, pubDate):
		self.items.append([title, author, pubDate])
	def output(self):
		self.req.content_type = 'application/xml'
		self.req.write("""<?xml version="1.0" encoding="utf-8"?>
<rss version="2.0">
<channel>
<title>%s</title>
<description>%s</description>
<link>%s</link>\n""" % (self.title, self.desc, self.link))
		for title, author, pubDate in self.items:
			self.req.write("""<item>
	<title>%s</title>\n""" % title)
			if author:
				self.req.write("\t<author>%s</author>\n" % author)
			if pubDate:
				self.req.write("\t<pubDate>%s</pubDate>\n" % formatdate(pubDate, True))
			self.req.write("</item>\n")
		self.req.write("</channel>\n</rss>")
		return apache.OK

class HostList:
	def __init__(self, hostlist):
		self.list = []

		for k, v in hostlist.items():
			if ping(v):
				self.list.append(k)

	def compare(self, list):
		joined = []
		left = list[:]
		for i in self.list:
			try:
				left.remove(i)
			except ValueError:
				joined.append(i)
		return joined, left

class HostChange:
	"""type: true if join, false if part"""
	def __init__(self, host, type):
		self.host = host
		self.type = type
		self.date = time.mktime(time.localtime())

class HostChanges:
	def __init__(self):
		self.state = []
		self.changes = []
		self.limit = 10
	def addjoins(self, joined):
		self.additems(joined, True)
	def addlefts(self, lefts):
		self.additems(lefts, False)
	def additems(self, list, type):
		for i in list:
			self.changes.append(HostChange(i, type))
	def debug(self):
		print "current state"
		print self.state
		print "recent events"
		for i in self.changes:
			if i.type:
				print "%s: %s joined" % (time.asctime(time.localtime(i.date)), i.host)
			else:
				print "%s: %s left" % (time.asctime(time.localtime(i.date)), i.host)
	def dump(self, socket):
		if len(self.changes) > self.limit:
			self.changes = self.changes[len(self.changes)-self.limit:]
		pickle.dump(self, socket)
	def torss(self, selfurl, req):
		rss = Rss(req, "Host monitor", selfurl,
				"This feed monitors host changes.")
		revchans = self.changes[:]
		revchans.reverse()
		for i in revchans:
			if i.type:
				rss.additem("available", i.host, i.date)
			else:
				rss.additem("no longer available", i.host, i.date)
		return rss.output()

class HostMon:
	def __init__(self, hostlist, dumpfile, myurl):
		self.hostlist = hostlist
		self.myurl = myurl
		try:
			socket = open(dumpfile, "r")
			changes = pickle.load(socket)
			socket.close()
		except IOError:
			changes = HostChanges()
		self.changes = changes

		hosts = HostList(hostlist)
		joined, left = hosts.compare(changes.state)

		changes.addjoins(joined)
		changes.addlefts(left)
		changes.state = hosts.list

		try:
			socket = open(dumpfile, "w")
			changes.dump(socket)
			socket.close()
		except IOError:
			pass

	def dorss(self, req):
		return self.changes.torss(self.myurl, req)

def handler(req):
	mon = HostMon({"factory":"factory.frugalware.org",
		"helicon":"helicon.frugalware.org",
		"bugs":"frugalware.hu",
		"wiki":"linuxforum.hu"},
		"/home/vmiklos/public_html/ping2rss/changes",
		"http://frugalware.org/~vmiklos/ping2rss")
	return mon.dorss(req)
