#!/usr/bin/env python

__author__ = "Miklos Vajna <vmiklos@frugalware.org>"
__version__ = "0.1"
__date__ = "Tue, 21 Nov 2006 22:54:11 +0100"
__copyright__ = "Copyright (c) 2006 Miklos Vajna"
__license__ = "GPL"

from mod_python import apache
from email.Utils import formatdate
import pickle, telnetlib, time

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

class NickList:
	def __init__(self, host, port, serverport):
		self.list = []

		server = telnetlib.Telnet(host, port)
		server.write("sel %d\n" % serverport)
		server.write("pl\n")
		server.write("quit\n")
		ret = server.read_all()
		server.close()
		for i in ret.split("\n")[3:-2]:
			self.list.append(i.split("\t")[14].strip('"'))

	def compare(self, list):
		joined = []
		left = list[:]
		for i in self.list:
			try:
				left.remove(i)
			except ValueError:
				joined.append(i)
		return joined, left

class NickChange:
	"""type: true if join, false if part"""
	def __init__(self, nick, type):
		self.nick = nick
		self.type = type
		self.date = time.mktime(time.localtime())

class NickChanges:
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
			self.changes.append(NickChange(i, type))
	def debug(self):
		print "current state"
		print self.state
		print "recent events"
		for i in self.changes:
			if i.type:
				print "%s: %s joined" % (time.asctime(time.localtime(i.date)), i.nick)
			else:
				print "%s: %s left" % (time.asctime(time.localtime(i.date)), i.nick)
	def dump(self, socket):
		if len(self.changes) > self.limit:
			self.changes = self.changes[len(self.changes)-self.limit:]
		pickle.dump(self, socket)
	def torss(self, tsurl, selfurl, req):
		rss = Rss(req, "TS monitor for %s" % tsurl, "%s" % selfurl,
				"This feed monitors the changes on the %s TeamSpeak server."%tsurl)
		revchans = self.changes[:]
		revchans.reverse()
		for i in revchans:
			if i.type:
				rss.additem("online", i.nick, i.date)
			else:
				rss.additem("offline", i.nick, i.date)
		return rss.output()

class TsMon:
	def __init__(self, host, port, serverport, dumpfile, myurl, req):
		try:
			socket = open(dumpfile, "r")
			changes = pickle.load(socket)
			socket.close()
		except IOError:
			changes = NickChanges()

		nicks = NickList(host, port, serverport)
		joined, left = nicks.compare(changes.state)

		changes.addjoins(joined)
		changes.addlefts(left)
		changes.state = nicks.list

		try:
			socket = open(dumpfile, "w")
			changes.dump(socket)
			socket.close()
		except IOError:
			pass

		return changes.torss(host, myurl, req)

if __name__ == "tsmon":
	def handler(req):
		TsMon("awnet.hu", 51234, 8767,
				"/home/vmiklos/public_html/tsmon/changes",
				"http://frugalware.org/~vmiklos/tsmon", req)
