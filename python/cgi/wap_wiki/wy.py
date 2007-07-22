#!/usr/bin/python

import re, os, os.path, cgi; env, ex = os.getenv, os.path.exists

cache = True

def load(n): return (ex('w/'+n) and open('w/'+n).read()) or ''
def fs(s): return reduce(lambda s, r: re.sub('(?m)'+r[0], r[1], s), (('\r',''),
	('(^|[^A-Za-z0-9?])(([A-Z][a-z]+){2,})', lambda m: (m.group(1) + '%s<a hr' \
	 'ef="wy.py?%s'+m.group(2)+'%s">%s</a>') % ((m.group(2),'p=','&amp;q=e','?'),
	('','','',m.group(2)))[ex('w/'+m.group(2))]),  ('^\{\{$','\n'),
	('^\* ','<br /> * '),  ('^}}$','<br />'),  ('^---$','<hr />'),  ('\n\n','<br />'),
	('(ht|f)tp:[^<>"\s]+','<a href="\g<0>">\g<0></a>')), cgi.escape(s or " "))
def do(m, n): return {'get':"""
	<a href="wy.py?p=%s&amp;q=f">%s</a><br />
	<a href="wy.py?p=%s&amp;q=e">edit me</a><br />
	%s
	""" % (n, n, n, fs(load(n)) or n),
	'edit': """
	%s
	<input type="text" name="t_%s" value="%s" /><br/>
	<anchor>[post it]
	<go method="post" href="wy.py?%s">
	<postfield name="p" value="%s"/>
	<postfield name="t_%s" value="$(t_%s)"/>
	</go>
	</anchor>
	""" % (n, n, load(n) or " ", n, n, n, n), 'find':
	("""
	Links: %s
	""" % fs(n)) +
	fs('{{\n* %s\n}}' % '\n* '.join(filter(
	 lambda s: open('w/'+s).read().find(n) > -1, os.listdir('w')))) +
	""}.get(m)
def main(f):
	n = f.get('p') or env("QUERY_STRING") or ''
	n = ('HomePage',n)[n.isalpha()]
	print "Content-Type: text/vnd.wap.wml"
	if not cache:
		print "Cache-Control: no-cache, must-revalidate"
		print "Pragma: no-cache"
	print ""
	print """<?xml version="1.0"?>
<!DOCTYPE wml PUBLIC "-//WAPFORUM//DTD WML 1.1//EN"
"http://www.wapforum.org/DTD/wml_1.1.xml">

<wml>
<card id="XML" title="%s">
<p>
	""" % n
	if env("REQUEST_METHOD") == "POST":
		open('w/'+n, 'w').write(f['t_%s' % n])
		os.chmod('w/'+n, 0646)
	print do({'e':'edit', 'f':'find'}.get(f.get('q')) or 'get', n)
if __name__=="__main__":
	main(dict([(i.name, i.value) for i in cgi.FieldStorage().list]))
	print """
</p>
</card>
</wml>
	"""
