CLANGDEFS=-D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -fno-rtti
CLANGWARNS=-Werror -Wall -Wno-missing-braces -Wnon-virtual-dtor -Wendif-labels -Wextra -Wundef -Wunused-macros -Wshadow -Woverloaded-virtual
CLANGFLAGS = $(CLANGDEFS) $(CLANGWARNS) -g
CLANGLIBS = -lclang-cpp -lLLVM

all: bin/readability-avoid-auto bin/bugprone-suspicious-xmlsec

bin/readability-avoid-auto: readability-avoid-auto.cxx Makefile
	clang++ $(CLANGFLAGS) $(CLANGLIBS) -ldl -lpthread -o $@ $<

bin/ast-matcher: ast-matcher.cxx Makefile
	clang++ $(CLANGFLAGS) $(CLANGLIBS) -ldl -lpthread -o $@ $<

bin/bugprone-suspicious-xmlsec: bugprone-suspicious-xmlsec.cxx Makefile
	clang++ $(CLANGFLAGS) $(CLANGLIBS) -ldl -lpthread -o $@ $<

demo:
	mkdir -p clang
	bin/readability-avoid-auto -export-fixes=clang/fixes.yaml qa/readability-avoid-auto.cxx -- -std=c++11
	clang-apply-replacements -remove-change-desc-files clang/

# vim: set noet sw=4 ts=4:
