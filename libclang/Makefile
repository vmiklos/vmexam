FLAGS := $(shell llvm-config --cxxflags --ldflags) -lclang

get-completion-at: get-completion-at.cxx Makefile
	clang++ $(FLAGS) -o get-completion-at get-completion-at.cxx

get-current-function: get-current-function.cxx Makefile
	clang++ $(FLAGS) -o get-current-function get-current-function.cxx

test:
	./get-completion-at test-completion.cxx 25 7 -std=c++1y
