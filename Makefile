true := T
false :=
RUST_PACKAGES :=
RUST_COVERED_PACKAGES :=

build:
	$(info make: ok for $(words $(RUST_PACKAGES)) Rust packages)

check:
	$(info make check: ok)

coverage-info:
	$(info $(words $(RUST_COVERED_PACKAGES)) covered packages: $(RUST_COVERED_PACKAGES))
	$(info $(words $(filter-out $(RUST_COVERED_PACKAGES),$(RUST_PACKAGES))) not covered packages: $(filter-out $(RUST_COVERED_PACKAGES),$(RUST_PACKAGES)))

install-git-hooks:
	cd .git/hooks && ln -sf ../../bash/clang-format-check commit-msg

%.check-test:
	$(if $(COVERAGE), cd $* && cargo llvm-cov --lib --ignore-filename-regex '(serde|system).rs' --show-missing-lines --fail-under-lines 100 -- --test-threads=1)
	$(if $(TEST), cd $* && cargo test --lib)


# $(call RustPackage_RustPackage,path)
define RustPackage_RustPackage
RUST_PACKAGES+= $(1)
build: $(1)
check: $(1)
check: $(1).check-doc
check: $(1).check-rustfmt
check: $(1).check-clippy
check: $(1).check-test

$(1).check-doc:
	test -f $(1)/README.md

$(1).run:
	cd $(1) && cargo run

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
$(1).check-test : TEST := $(false)

endef

# $(call RustPackage_use_coverage,path)
define RustPackage_use_coverage
RUST_COVERED_PACKAGES+= $(1)
$(1).check-test : COVERAGE := $(true)

endef

# $(call RustPackage_use_test,path)
define RustPackage_use_test
$(1).check-test : TEST := $(true)

endef

$(eval $(call RustPackage_RustPackage,avg))

$(eval $(call RustPackage_RustPackage,binary-dumper))

$(eval $(call RustPackage_RustPackage,cap2exif))

$(eval $(call RustPackage_RustPackage,csp))
$(eval $(call RustPackage_use_test,csp))

$(eval $(call RustPackage_RustPackage,darcs-git))
$(eval $(call RustPackage_use_coverage,darcs-git))

$(eval $(call RustPackage_RustPackage,fit2json))

$(eval $(call RustPackage_RustPackage,git-ls-projects))

$(eval $(call RustPackage_RustPackage,git-review-link))

$(eval $(call RustPackage_RustPackage,github-outdated))

$(eval $(call RustPackage_RustPackage,hyphen))
$(eval $(call RustPackage_use_test,hyphen))

$(eval $(call RustPackage_RustPackage,hyphen-sys))
$(eval $(call RustPackage_use_test,hyphen-sys))

$(eval $(call RustPackage_RustPackage,ics2txt))
$(eval $(call RustPackage_use_coverage,ics2txt))

$(eval $(call RustPackage_RustPackage,markdown-stats))

$(eval $(call RustPackage_RustPackage,mso-convert))

$(eval $(call RustPackage_RustPackage,mutt-display-filter))

$(eval $(call RustPackage_RustPackage,mutt-imap-lister))

$(eval $(call RustPackage_RustPackage,nextcloud-open))
$(eval $(call RustPackage_use_coverage,nextcloud-open))

$(eval $(call RustPackage_RustPackage,notmuch-showref))

$(eval $(call RustPackage_RustPackage,ocr-document))

$(eval $(call RustPackage_RustPackage,osm/addr-osmify-rust))
$(eval $(call RustPackage_use_coverage,osm/addr-osmify-rust))

$(eval $(call RustPackage_RustPackage,pdfcal))

$(eval $(call RustPackage_RustPackage,pushping))

$(eval $(call RustPackage_RustPackage,rsp-scramble))

$(eval $(call RustPackage_RustPackage,rubik))

$(eval $(call RustPackage_RustPackage,scan-document))

$(eval $(call RustPackage_RustPackage,send-email))

$(eval $(call RustPackage_RustPackage,share-vmiklos-hu-apps))

$(eval $(call RustPackage_RustPackage,ssh-proxy))

$(eval $(call RustPackage_RustPackage,tpconv))
$(eval $(call RustPackage_use_coverage,tpconv))

$(eval $(call RustPackage_RustPackage,uchroot))

$(eval $(call RustPackage_RustPackage,weechat-calc))

$(eval $(call RustPackage_RustPackage,weesearch))
$(eval $(call RustPackage_use_coverage,weesearch))
