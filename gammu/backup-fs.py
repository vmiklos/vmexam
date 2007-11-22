#!/usr/bin/env python

import os, re, shutil, time

bd = "filesystem-%s" % time.strftime("%Y%m%d", time.localtime())
try:
	shutil.rmtree(bd)
except OSError:
	pass
os.mkdir(bd)
os.chdir(bd)

sock = os.popen("gammu --getfilesystem -flatall")

buf = sock.readlines()

sock.close()

for i in buf:
	if not re.search("File", i):
		continue
	id = i.split(";")[0]
	folder = i.split(";")[2].strip('"')
	file = i.split(";")[3]

	try:
		os.mkdir(folder)
	except OSError:
		pass

	cwd = os.getcwd()
	os.chdir(folder)
	os.system("gammu --getfiles %s" % id)
	os.chdir(cwd)
