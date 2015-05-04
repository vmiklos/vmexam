CLANGDEFS=-D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -fno-rtti
CLANGWARNS=-Werror -Wall -Wno-missing-braces -Wnon-virtual-dtor -Wendif-labels -Wextra -Wundef -Wunused-macros -Wshadow -Woverloaded-virtual
CLANGFLAGS = $(CLANGDEFS) $(CLANGWARNS) -g -std=c++11
CLANGLIBS = -lLLVMSupport -lclangAST -lclangBasic -lclangFrontend -lclangRewrite -lclangTooling

bin/rename: rename.cxx Makefile
	clang++ $(CLANGFLAGS) $(CLANGLIBS) -o $@ rename.cxx

# vim: set noet sw=4 ts=4:
