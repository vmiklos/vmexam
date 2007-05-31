#!/usr/bin/env python

import cgitb, cgi, re, socket, os, time, Cookie, sha, sys, urllib
from distutils import sysconfig

cgitb.enable()
last = None

class Pyrssi:
	def __init__(self, sock_path, passwd):
		self.sock_path = sock_path
		self.passwd = passwd
		self.year = 60*60*24*365

	def send(self, what):
		try:
			self.cookie = Cookie.SimpleCookie(os.environ["HTTP_COOKIE"])
		except KeyError:
			self.cookie = Cookie.SimpleCookie()
		if 'pyrssi_network' in self.cookie.keys():
			self.network = self.cookie['pyrssi_network'].value
		else:
			self.network = None
		if 'pyrssi_refnum' in self.cookie.keys():
			self.refnum = self.cookie['pyrssi_refnum'].value
		else:
			self.refnum = None
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
		elif "pyrssi_channel" not in self.cookie.keys():
			self.__dumpwindowlist()
			self.__sysinfo()
		else:
			self.__dumpform()
			self.__dumplastlines()
		if "pyrssi_pass" in self.cookie.keys():
			self.__dumplogout()
		self.__dumpfooter()

	def __sysinfo(self):
		print "Powered by Python %s (on %s/%s)" % (sysconfig.get_python_version(), sys.platform, os.name)
		print "<br />"

	def __handlecookies(self):
		# see if we should set cookies
		if "action" in self.form.keys() and self.form['action'].value == "login":
			if sha.sha(self.form['pass'].value).hexdigest() != self.passwd:
				self.__dumpheader()
				self.__dumplogin("wrong password!<br />")
				self.__dumpfooter()
				sys.exit(0)
			self.cookie = Cookie.SimpleCookie()
			self.cookie['pyrssi_pass'] = self.form['pass'].value
			self.cookie['pyrssi_pass']['max-age'] = self.year
			print self.cookie

		if "action" in self.form.keys() and self.form['action'].value == "logout":
			self.cookie['pyrssi_pass']['max-age'] = 0
			print self.cookie
			self.cookie = {}

		if "action" in self.form.keys() and self.form['action'].value == "windowselect":
			self.cookie['pyrssi_channel'] = self.form['window'].value.lower()
			self.cookie['pyrssi_channel']['max-age'] = self.year
			self.cookie['pyrssi_network'] = self.form['network'].value
			self.cookie['pyrssi_network']['max-age'] = self.year
			self.cookie['pyrssi_refnum'] = self.form['refnum'].value
			self.cookie['pyrssi_refnum']['max-age'] = self.year
			print self.cookie
			self.channel = self.form['window'].value.lower()
			self.network = self.form['network'].value
			self.refnum = self.form['refnum'].value

		if "action" in self.form.keys() and self.form['action'].value == "windowlist":
			if 'pyrssi_channel' in self.cookie.keys():
				self.cookie['pyrssi_channel']['max-age'] = 0
				print self.cookie
				del self.cookie['pyrssi_channel']

	def __connect(self):
		self.sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
		self.sock.connect(self.sock_path)

	def __send(self, what):
		ret = 0
		if len(what):
			if what.startswith("/g "):
				for i in self.__recv("windowlist").split("\n"):
					refnum = re.sub(r'(.*): .*', r'\1', i)
					window = re.sub(r'.*: (.*) \(.*', r'\1', i)
					network = re.sub(r'.* \((.*)\).*', r'\1', i)
					if refnum == what[3:] or window == what[3:]:
						self.cookie['pyrssi_channel'] = window.lower()
						self.cookie['pyrssi_channel']['max-age'] = self.year
						self.cookie['pyrssi_network'] = network
						self.cookie['pyrssi_network']['max-age'] = self.year
						self.cookie['pyrssi_refnum'] = refnum
						self.cookie['pyrssi_refnum']['max-age'] = self.year
						print self.cookie
						self.channel = window
						self.network = network.lower()
						self.refnum = refnum
						break
			else:
				self.__connect()
				ret += self.sock.send("switch %s" % self.refnum)
				self.__connect()
				ret += self.sock.send("send %s" % what)
				time.sleep(0.5)
		return ret

	def __recv(self, what):
		ret = []
		self.__connect()
		self.sock.send(what)
		while True:
			buf = self.sock.recv(4096)
			if not buf:
				break
			ret.append(buf)
		return "".join(ret)

	def __getlastlines(self):
		ret = []
		buf = self.__recv("get_lines %s" % self.refnum)
		return buf.split("\n")[-25:]

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

	def __dumplogin(self, errmsg=""):
		print """
		%s
		password: <input type="password" name="pass" value="" /><br/>
		<anchor>[login]
		<go method="post" href="pyrssi.py">
		<postfield name="pass" value="$(pass)"/>
		<postfield name="action" value="login"/>
		</go>
		</anchor>""" % errmsg
	def __dumpwindowlist(self):
		for i in self.__recv("windowlist").split("\n"):
			refnum = re.sub(r'(.*): .*', r'\1', i)
			window = re.sub(r'.*: (.*) \(.*', r'\1', i)
			network = re.sub(r'.* \((.*)\).*', r'\1', i)
			print """<a href="pyrssi.py?action=windowselect&amp;refnum=%s&amp;window=%s&amp;network=%s">%s</a><br />""" % (refnum, urllib.pathname2url(window), network, cgi.escape(window))
	def __dumpform(self):
		print """<input type="text" name="msg" value="" /><br/>
		<anchor>[send]
		<go method="post" href="pyrssi.py">
		<postfield name="msg" value="$(msg)"/>
		<postfield name="action" value="msg"/>
		</go>
		</anchor>"""
		print """
		<anchor>[windowlist]
		<go method="post" href="pyrssi.py">
		<postfield name="action" value="windowlist"/>
		</go>
		</anchor>"""
	def __dumplogout(self):
		print """
		<anchor>[logout]
		<go method="post" href="pyrssi.py">
		<postfield name="action" value="logout"/>
		</go>
		</anchor>
		<br />"""

	def __dumpfooter(self):
		print """</p>
		</card>
		</wml>"""

	def __dumplastlines(self):
		self.lastlines = self.__getlastlines()
		self.lastlines.reverse()
		for i in self.lastlines:
			print cgi.escape(i),  '<br />'

# pass is foo for now
pyrssi = Pyrssi('/home/vmiklos/.irssi/socket', '502b57ea9f1d731a9a63cb16b6aeb3358a8973d1')
pyrssi.send(cgi.FieldStorage())
pyrssi.receive()
