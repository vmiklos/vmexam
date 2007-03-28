#!/usr/bin/env python

import cgitb

cgitb.enable()

import cgi, Cookie, os, sha, sys

form = cgi.FieldStorage()
# see if we should set cookies
try:
	c = Cookie.SimpleCookie(os.environ["HTTP_COOKIE"])
except KeyError:
	c = Cookie.SimpleCookie()
if "action" in form.keys() and form['action'].value == "login":
	# pass is foo for now
	if form['user'].value == "john" and \
			sha.sha(form['pass'].value).hexdigest() != '0beec7b5ea3f0fdbc95d0dd47f3c5bc275da8a33':
		sys.exit(0)
	year = 60*60*24*365
	c = Cookie.SimpleCookie()
	c['example_user'] = form['user'].value
	c['example_user']['max-age'] = year
	c['example_pass'] = form['pass'].value
	c['example_pass']['max-age'] = year
	print c

if "action" in form.keys() and form['action'].value == "logout":
	c['example_user']['max-age'] = 0
	c['example_pass']['max-age'] = 0
	print c
	c = Cookie.SimpleCookie()

print "Content-Type: text/vnd.wap.wml"
print "Cache-Control: no-cache, must-revalidate"
print "Pragma: no-cache"
print
print """<?xml version="1.0"?>
<!DOCTYPE wml PUBLIC "-//WAPFORUM//DTD WML 1.1//EN"
"http://www.wapforum.org/DTD/wml_1.1.xml">
<wml>
<card id="XML" title="example">
<p>"""

if "example_pass" not in c.keys():
	print """
	username: <input type="text" name="user" value="" /><br/>
	password: <input type="text" name="pass" value="" /><br/>
	<anchor>[login]
	<go method="post" href="cookie.py">
	<postfield name="pass" value="$(pass)"/>
	<postfield name="user" value="$(user)"/>
	<postfield name="action" value="login"/>
	</go>
	</anchor>"""
else:
	print "user: %s<br/>" % c['example_user'].value
	print """<anchor>[logout]
	<go method="post" href="cookie.py">
	<postfield name="action" value="logout"/>
	</go>
	</anchor><br />"""
print """</p>
</card>
</wml>"""
