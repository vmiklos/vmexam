CLANGDIR= $(HOME)/git/libreoffice/dev-tools/clang
check:
	rm -f $(CLANGDIR)/bin/rename
	make -C $(CLANGDIR) GCOV=y bin/rename
	qa/test-rename.sh
