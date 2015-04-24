CLANGDEFS=-D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -fno-rtti
CLANGWARNS=-Werror -Wall -Wno-missing-braces -Wnon-virtual-dtor -Wendif-labels -Wextra -Wundef -Wunused-macros -Wshadow -Woverloaded-virtual
# TODO check if all of these are necessary
CLANGLIBS=$(shell llvm-config --libs) -lclangAnalysis -lclangARCMigrate -lclangASTMatchers -lclangAST -lclangBasic -lclangCodeGen -lclangDriver -lclangDynamicASTMatchers -lclangEdit -lclangFormat -lclangFrontend -lclangFrontendTool -lclangIndex -lclangLex -lclangParse -lclangRewriteFrontend -lclangRewrite -lclangSema -lclangSerialization -lclangStaticAnalyzerCheckers -lclangStaticAnalyzerCore -lclangStaticAnalyzerFrontend -lclangTooling

rename: rename.cxx Makefile
	clang++ -g $(CLANGWARNS) $(CLANGDEFS) -std=c++11 $(CLANGLIBS) -o rename rename.cxx
