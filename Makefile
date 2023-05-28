# $(call RustPackage,path)
define RustPackage
build: build-$(1)
check: check-doc-$(1)
check: check-rustfmt-$(1)
check: check-clippy-$(1)

check-doc-$(1):
	ls $(1)/README.md >/dev/null

build-$(1):
	cd $(1) && cargo build

check-rustfmt-$(1):
	cd $(1) && cargo fmt -- --check

check-clippy-$(1):
	cd $(1) && cargo clippy

.PHONY: $(1)
.PHONY: $(1).check

$(1): build-$(1)
$(1).check: $(1) check-doc-$(1) check-rustfmt-$(1) check-clippy-$(1)
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
