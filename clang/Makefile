.PHONY : coverage

CLANGDIR= $(HOME)/git/libreoffice/dev-tools/clang
check:
	rm -f bin lib64
	ln -s $(CLANGDIR)/bin
	ln -s $(CLANGDIR)/lib64
	qa/test-rename.sh

coverage:
	rm -f $(CLANGDIR)/bin/rename
	make -C $(CLANGDIR) GCOV=y bin/rename
	make check
	lcov --directory $(CLANGDIR) --gcov-tool $(PWD)/qa/llvm-gcov.sh --capture --output-file clang.info
	genhtml -o coverage clang.info
