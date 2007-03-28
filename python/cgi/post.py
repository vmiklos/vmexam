#!/usr/bin/env python

import cgitb

cgitb.enable()

import cgi

print "Content-Type: text/vnd.wap.wml"
print "Cache-Control: no-cache, must-revalidate"
print "Pragma: no-cache"
print
print """<?xml version="1.0"?>
<!DOCTYPE wml PUBLIC "-//WAPFORUM//DTD WML 1.1//EN"
"http://www.wapforum.org/DTD/wml_1.1.xml">
<wml>
<card id="XML" title="pyrssi">
<p>"""
form = cgi.FieldStorage()
if len(form):
	print form['foo'].value, "<br />"
print """
	<input type="text" name="foo" value="bar" /><br/>
	<anchor>[send]
	<go method="post" href="test.py">
	<postfield name="foo" value="$(foo)"/>
	</go>
	</anchor>
</p>
</card>
</wml>"""
