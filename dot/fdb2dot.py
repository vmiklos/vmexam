#!/usr/bin/env python

try:
	import pacman
except ImportError:
	import alpm
	pacman = alpm
import os, tempfile, shutil, sys, time

def dotify(s):
	return "pkg_" + s.replace("+", "plus").replace("-", "_").replace("@", "at").replace(".", "dot")

print "digraph {"
print 'label="Frugalware 0.8 i686 dependencies for the \'base\' group"'
print 'ratio=1.41'
root = tempfile.mkdtemp()
if pacman.initialize(root) == -1:
	print "initialize() failed"
treename = sys.argv[1]
local = pacman.db_register("local")
db = pacman.db_register(treename)
if not db:
	print "db_register() failed"
if pacman.db_setserver(db, "file://" + os.getcwd()) == -1:
	print "db_setserver() failed"
if pacman.db_update(1, db) == -1:
	print "db_update() failed"
if pacman.trans_init(pacman.TRANS_TYPE_SYNC, pacman.TRANS_FLAG_NOCONFLICTS, None, None, None) == -1:
	print "trans_init() failed"
i = pacman.db_getpkgcache(db)
while i:
	pkg = pacman.void_to_PM_PKG(pacman.list_getdata(i))
	pkgname = pacman.void_to_char(pacman.pkg_getinfo(pkg, pacman.PKG_NAME))
	pkgver = pacman.void_to_char(pacman.pkg_getinfo(pkg, pacman.PKG_VERSION))
	group = pacman.void_to_char(pacman.list_getdata(pacman.void_to_PM_LIST(pacman.pkg_getinfo(pkg, pacman.PKG_GROUPS))))
	if group != "base":
		i = pacman.list_next(i)
		continue
	print '%s [label="%s %s"]' % (dotify(pkgname), pkgname, pkgver)
	j = pacman.void_to_PM_LIST(pacman.pkg_getinfo(pkg, pacman.PKG_DEPENDS))
	while j:
		dep = pacman.void_to_char(pacman.list_getdata(j)).split("<")[0].split(">")[0].split("=")[0]
		print "%s->%s" % (dotify(dep), dotify(pkgname))
		j = pacman.list_next(j)
	i = pacman.list_next(i)
print "}"
pacman.release()
shutil.rmtree(root)
