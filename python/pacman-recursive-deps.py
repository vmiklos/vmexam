#!/usr/bin/env python
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
#

import pacman
import sys

def get_recursive_deps(name, result):
	i = pacman.db_getpkgcache(db_local)
	while i:
		pkg = pacman.void_to_PM_PKG(pacman.list_getdata(i))
		pkgname = pacman.void_to_char(pacman.pkg_getinfo(pkg, pacman.PKG_NAME))
		if pkgname == name:
			j = pacman.void_to_PM_LIST(pacman.pkg_getinfo(pkg, pacman.PKG_DEPENDS))
			while j:
				depname = pacman.void_to_char(pacman.list_getdata(j)).split("<")[0].split(">")[0].split("=")[0]
				if depname not in result:
					result.append(depname)
					get_recursive_deps(depname, result)
				j = pacman.list_next(j)
		i = pacman.list_next(i)

pacman.initialize("/") == 0
db_local = pacman.db_register("local")
result = []
get_recursive_deps(sys.argv[1], result)
print "\n".join(result)
pacman.release() == 0

# vim:set shiftwidth=4 softtabstop=4 expandtab:
