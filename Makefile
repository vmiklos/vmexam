check: check-gammu check-gcontacts check-osm check-mirror

check-gammu:
	cd gammu && $(MAKE) check

check-gcontacts:
	cd gcontacts && $(MAKE) check

check-osm:
	cd osm && $(MAKE) check

check-mirror:
	cd mirror && $(MAKE) check

check-doc:
	for i in  */Cargo.toml; do cd $$(dirname $$i); ls README.md &>/dev/null|| echo "$$i: no doc"; cd - >/dev/null; done

hooks:
	cd .git/hooks && ln -sf ../../bash/clang-format-check pre-commit
