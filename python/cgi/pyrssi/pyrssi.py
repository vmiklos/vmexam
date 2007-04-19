#!/usr/bin/env python

import cgitb, cgi, re, socket, os, time, Cookie, sha, sys

cgitb.enable()
last = None

class Pyrssi:
	def __init__(self):
		self.sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
		self.sock.connect(("/tmp/irssi_socket"))

	def send(self, what = "hm from irssi-cmd.py"):
		try:
			self.cookie = Cookie.SimpleCookie(os.environ["HTTP_COOKIE"])
		except KeyError:
			self.cookie = Cookie.SimpleCookie()
		if 'pyrssi_network' in self.cookie.keys():
			self.network = self.cookie['pyrssi_network'].value
		else:
			self.network = None
		if 'pyrssi_channel' in self.cookie.keys():
			self.channel = self.cookie['pyrssi_channel'].value
		else:
			self.channel = None
		if isinstance(what, cgi.FieldStorage):
			self.form = what
			try:
				self.__send(self.form['msg'].value)
			except KeyError:
				pass
		else:
			self.__send(what)

	def receive(self):
		self.__handlecookies()
		self.__dumpheader()
		if "pyrssi_pass" not in self.cookie.keys():
			self.__dumplogin()
		else:
			self.__dumpform()
			self.__dumplastlines()
			self.__dumplogout()
		self.__dumpfooter()

	def __handlecookies(self):
		# see if we should set cookies
		if "action" in self.form.keys() and self.form['action'].value == "login":
			# foo for now
			if sha.sha(self.form['pass'].value).hexdigest() != '502b57ea9f1d731a9a63cb16b6aeb3358a8973d1':
				sys.exit(0)
			year = 60*60*24*365
			self.cookie = Cookie.SimpleCookie()
			self.cookie['pyrssi_pass'] = self.form['pass'].value
			self.cookie['pyrssi_pass']['max-age'] = year
			self.cookie['pyrssi_network'] = self.form['network'].value
			self.cookie['pyrssi_network']['max-age'] = year
			self.cookie['pyrssi_channel'] = self.form['channel'].value
			self.cookie['pyrssi_channel']['max-age'] = year
			print self.cookie
			if not self.network:
				self.network = self.form['network'].value
			if not self.channel:
				self.channel = self.form['channel'].value

		if "action" in self.form.keys() and self.form['action'].value == "logout":
			self.cookie['pyrssi_pass']['max-age'] = 0
			self.cookie['pyrssi_network']['max-age'] = 0
			self.cookie['pyrssi_channel']['max-age'] = 0
			print self.cookie
			self.cookie = {}

	def __send(self, what):
		if len(what):
			ret = self.sock.send("command msg -%s %s %s" % (self.network, self.channel, what))
			time.sleep(0.5)
		return ret

	def __getlastfile(self):
		last = None
		newest = 0
		for root, dirs, files in os.walk(self.logpath):
			for file in files:
				if re.sub(r'_[0-9]{8}.log', '', file) == self.channel:
					date = int(file[len(self.channel)+1:-4])
					if date > newest:
						newest = date
						last = file
		return last

	def __getlastlines(self):
		ret = []
		sock = open(os.path.join(self.logpath, time.strftime("%Y"), self.last))
		buf = sock.read()
		for i in buf.split("\n")[-40:]:
			if not re.match(r"^--- Log", i):
				ret.append(i)
		return ret

	def __dumpheader(self):
		print "Content-Type: text/vnd.wap.wml"
		print "Cache-Control: no-cache, must-revalidate"
		print "Pragma: no-cache"
		print
		print """<?xml version="1.0"?>
		<!DOCTYPE wml PUBLIC "-//WAPFORUM//DTD WML 1.1//EN"
		"http://www.wapforum.org/DTD/wml_1.1.xml">
		<wml>
		<card id="XML" title="%s">
		<p>""" % time.strftime("[%H:%M]")

	def __dumplogin(self):
		print """
		password: <input type="password" name="pass" value="" /><br/>
		network: <input type="text" name="network" value="" /><br/>
		channel: <input type="text" name="channel" value="" /><br/>
		<anchor>[login]
		<go method="post" href="pyrssi.py">
		<postfield name="pass" value="$(pass)"/>
		<postfield name="network" value="$(network)"/>
		<postfield name="channel" value="$(channel)"/>
		<postfield name="action" value="login"/>
		</go>
		</anchor>"""
	def __dumpform(self):
		print """<input type="text" name="msg" value="" /><br/>
		<anchor>[send]
		<go method="post" href="pyrssi.py">
		<postfield name="msg" value="$(msg)"/>
		<postfield name="action" value="msg"/>
		</go>
		</anchor>"""
	def __dumplogout(self):
		print """
		<anchor>[logout]
		<go method="post" href="pyrssi.py">
		<postfield name="action" value="logout"/>
		</go>
		</anchor><br />"""

	def __dumpfooter(self):
		print """</p>
		</card>
		</wml>"""

	def __dumplastlines(self):
		self.logpath = os.path.join("/home/vmiklos/.irssi/logs", self.network)
		self.last = self.__getlastfile()
		self.lastlines = self.__getlastlines()
		self.lastlines.reverse()
		for i in self.lastlines:
			print cgi.escape(i),  '<br />'

pyrssi = Pyrssi()
pyrssi.send(cgi.FieldStorage())
pyrssi.receive()
