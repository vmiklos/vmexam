FLAGS := $(shell llvm-config --cxxflags --ldflags) -lclang

completion: completion.cxx Makefile
	clang++ $(FLAGS) -o completion completion.cxx

get-current-function: get-current-function.cxx Makefile
	clang++ $(FLAGS) -o get-current-function get-current-function.cxx
