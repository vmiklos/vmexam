import alpm

what = "/usr/bin/pacman"

alpm.initialize("/")
db = alpm.db_register('local')

i = alpm.db_getpkgcache(db)
while i:
	pkg = alpm.void_to_PM_PKG(alpm.list_getdata(i))
	j = alpm.void_to_PM_LIST(alpm.pkg_getinfo(pkg, alpm.PKG_FILES))
	while j:
		if alpm.void_to_char(alpm.list_getdata(j)) == what[1:]:
			print alpm.void_to_char(alpm.pkg_getinfo(pkg, alpm.PKG_NAME))
		j = alpm.list_next(j)
	i = alpm.list_next(i)
