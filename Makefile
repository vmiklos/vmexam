check: check-gammu check-gcontacts check-osm

check-gammu:
	cd gammu && $(MAKE) check

check-gcontacts:
	cd gcontacts && $(MAKE) check

check-osm:
	cd osm && $(MAKE) check
