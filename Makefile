# $(call RustPackage,path)
define RustPackage
build: $(1).build
check: $(1).check-doc
check: $(1).check-rustfmt
check: $(1).check-clippy

$(1).check-doc:
	ls $(1)/README.md >/dev/null

$(1).build:
	cd $(1) && cargo build

$(1).check-rustfmt:
	cd $(1) && cargo fmt -- --check

$(1).check-clippy:
	cd $(1) && cargo clippy

$(1).check-cov:
	cd $(1) && cargo llvm-cov --lib  --show-missing-lines --fail-under-lines 100 -- --test-threads=1

.PHONY: $(1)
.PHONY: $(1).check

$(1): $(1).build
$(1).check: $(1) $(1).check-doc $(1).check-rustfmt $(1).check-clippy
endef

$(eval $(call RustPackage,avg))
$(eval $(call RustPackage,csp))
$(eval $(call RustPackage,darcs-git))
$(eval $(call RustPackage,mutt-display-filter))
$(eval $(call RustPackage,mutt-imap-lister))
$(eval $(call RustPackage,nextcloud-open))
$(eval $(call RustPackage,notmuch-showref))
$(eval $(call RustPackage,osm/addr-osmify-rust))
$(eval $(call RustPackage,pushping))
$(eval $(call RustPackage,scan-document))
$(eval $(call RustPackage,share-vmiklos-hu-apps))
$(eval $(call RustPackage,ssh-proxy))
$(eval $(call RustPackage,weechat-calc))
