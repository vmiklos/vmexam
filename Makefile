SUBDIRS = gammu osm
check:
	for i in $(SUBDIRS); do $(MAKE) -C $$i check || exit 1; done
