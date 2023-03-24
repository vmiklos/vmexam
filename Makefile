check: check-gcontacts check-osm check-mirror

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

RUST_PROJECTS = \
	avg \
	mutt-display-filter \
	mutt-imap-lister \
	nextcloud-open \
	notmuch-showref \
	pushping \
	scan-document \
	share-vmiklos-hu-apps \
	ssh-proxy \
	weechat-calc \

check-build:
	for i in $(RUST_PROJECTS); do cd $$i; cargo build || exit 1; cd ..; done

check-rustfmt:
	for i in $(RUST_PROJECTS); do cd $$i; cargo fmt -- --check || exit 1; cd ..; done

check-clippy:
	for i in $(RUST_PROJECTS); do cd $$i; cargo clippy || exit 1; cd ..; done
