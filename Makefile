true := T
false :=

check:
	@echo "make check: ok"

%.check-test:
	$(if $(COVERAGE), cd $* && cargo llvm-cov --lib  --show-missing-lines --fail-under-lines 100 -- --test-threads=1)


# $(call RustPackage_RustPackage,path)
define RustPackage_RustPackage
build: $(1)
check: $(1).check-doc
check: $(1).check-rustfmt
check: $(1).check-clippy
check: $(1).check-test

$(1).check-doc:
	ls $(1)/README.md >/dev/null

$(1):
	cd $(1) && cargo build

$(1).check-rustfmt:
	cd $(1) && cargo fmt -- --check

$(1).check-clippy:
	cd $(1) && cargo clippy

.PHONY: $(1)
.PHONY: $(1).check

$(1).check: $(1) $(1).check-doc $(1).check-rustfmt $(1).check-clippy $(1).check-test
$(1).check-test : COVERAGE := $(false)

endef

define RustPackage_use_coverage
$(1).check-test : COVERAGE := $(true)

endef

$(eval $(call RustPackage_RustPackage,avg))
$(eval $(call RustPackage_RustPackage,csp))
$(eval $(call RustPackage_RustPackage,darcs-git))
$(eval $(call RustPackage_RustPackage,hyphen))
$(eval $(call RustPackage_RustPackage,hyphen-sys))
$(eval $(call RustPackage_RustPackage,mutt-display-filter))
$(eval $(call RustPackage_RustPackage,mutt-imap-lister))
$(eval $(call RustPackage_RustPackage,nextcloud-open))
$(eval $(call RustPackage_use_coverage,nextcloud-open))
$(eval $(call RustPackage_RustPackage,notmuch-showref))
$(eval $(call RustPackage_RustPackage,osm/addr-osmify-rust))
$(eval $(call RustPackage_RustPackage,pushping))
$(eval $(call RustPackage_RustPackage,scan-document))
$(eval $(call RustPackage_RustPackage,share-vmiklos-hu-apps))
$(eval $(call RustPackage_RustPackage,ssh-proxy))
$(eval $(call RustPackage_RustPackage,weechat-calc))
