check: check-gammu check-osm

check-gammu:
	cd gammu && $(MAKE) check

check-osm:
	cd osm && $(MAKE) check
