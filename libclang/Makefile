FLAGS := $(shell llvm-config --cxxflags --ldflags) -lclang

get-current-function: get-current-function.cxx Makefile
	clang++ $(FLAGS) -o get-current-function get-current-function.cxx
