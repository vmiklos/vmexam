import os, stat, sys

"""
this is a mailbox lister

it has 3 main features:

	1) lists all mailbox when running directly on a courier imap server

	2) hides big lists if you want so (for examply there can be lists to
	where you write once a month, wait for replies then you don't read the
	traffic for a month again)

	3) archives your mailboxes

for 2), there is a simple config, too
"""

def usage():
	print "usage: %s [--base|--big|--archive]" % sys.argv[0]
	sys.exit(1)

basedir = os.path.join(os.environ['HOME'], "Maildir")
os.chdir(basedir)

import config

l = config.big
hide = []
for i in l:
	if "_archive" not in i:
		hide.append(i)
hide.sort()

dirs = [""]
for i in os.listdir(basedir):
	if stat.S_ISDIR(os.stat(i)[stat.ST_MODE]) and i.startswith(".") and i not in hide:
		dirs.append(i)
dirs.sort()

if len(sys.argv) == 1:
	usage()
if sys.argv[1] == "--base":
	l = dirs
elif sys.argv[1] == "--big":
	l = hide
elif sys.argv[1] == "--archive":
	l = dirs
	l.extend(hide)
	l.sort()
	os.rename(os.path.join(basedir, "Maildir_archive"), os.path.join(os.environ['HOME'], "Maildir_archive"))
	for i in l:
		cmd = 'archivemail -q --no-compress "%s"' % os.path.join(basedir, i)
		if len(sys.argv) == 3:
			print cmd
		else:
			os.system(cmd)
	os.rename(os.path.join(os.environ['HOME'], "Maildir_archive"), os.path.join(basedir, "Maildir_archive"))
	sys.exit(0)
else:
	usage()

print " ".join([os.path.join(basedir, i) for i in l])
