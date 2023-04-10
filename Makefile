check: check-doc check-build check-rustfmt check-clippy

RUST_PROJECTS = \
	avg \
	csp \
	mutt-display-filter \
	mutt-imap-lister \
	nextcloud-open \
	notmuch-showref \
	pushping \
	scan-document \
	share-vmiklos-hu-apps \
	ssh-proxy \
	weechat-calc \

check-doc:
	for i in $(RUST_PROJECTS); do ls $$i/README.md >/dev/null || exit 1; done

check-build:
	for i in $(RUST_PROJECTS); do cd $$i; cargo build || exit 1; cd ..; done

check-rustfmt:
	for i in $(RUST_PROJECTS); do cd $$i; cargo fmt -- --check || exit 1; cd ..; done

check-clippy:
	for i in $(RUST_PROJECTS); do cd $$i; cargo clippy || exit 1; cd ..; done
