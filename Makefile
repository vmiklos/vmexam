check: check-gammu check-gcontacts check-osm check-mirror

check-gammu:
	cd gammu && $(MAKE) check

check-gcontacts:
	cd gcontacts && $(MAKE) check

check-osm:
	cd osm && $(MAKE) check

check-mirror:
	cd mirror && $(MAKE) check

hooks:
	cd .git/hooks && ln -sf ../../bash/clang-format-check pre-commit
