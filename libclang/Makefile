FLAGS := $(shell llvm-config --cxxflags --ldflags) -lclang -O0 -g -D_GLIBCXX_DEBUG

get-compiler-args: get-compiler-args.cxx Makefile
	clang++ $(FLAGS) -o get-compiler-args get-compiler-args.cxx

get-completion-at: get-completion-at.cxx Makefile
	clang++ $(FLAGS) -o get-completion-at get-completion-at.cxx

get-documentation-at: get-documentation-at.cxx Makefile
	clang++ $(FLAGS) -o get-documentation-at get-documentation-at.cxx

get-current-function: get-current-function.cxx Makefile
	clang++ $(FLAGS) -o get-current-function get-current-function.cxx

test:
	./get-completion-at test-completion.cxx 25 7 -std=c++1y
