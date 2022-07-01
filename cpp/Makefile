CXXFLAGS = -std=c++17 -g

json: json.cc Makefile
	g++ ${CXXFLAGS} -o json json.cc

json-array-read: json-array-read.cc Makefile
	g++ ${CXXFLAGS} -o json-array-read json-array-read.cc

poco: poco.cc Makefile
	g++ ${CXXFLAGS} -o poco poco.cc -lPocoFoundation -lPocoJSON

crtp: crtp.cc Makefile
	g++ ${CXXFLAGS} -o crtp crtp.cc

python-filter: python-filter.cxx Makefile
	g++ ${CXXFLAGS} -o python-filter python-filter.cxx

python-map: python-map.cxx Makefile
	g++ ${CXXFLAGS} -o python-map python-map.cxx

python-random-choice: python-random-choice.cxx Makefile
	g++ ${CXXFLAGS} -o python-random-choice python-random-choice.cxx

scope-guard: scope-guard.cxx Makefile
	g++ ${CXXFLAGS} -o scope-guard scope-guard.cxx

async: async.cxx Makefile
	g++ ${CXXFLAGS} -pthread -o async async.cxx

asan: asan.cxx Makefile
	clang++ ${CXXFLAGS} -o asan -O0 -fsanitize=address asan.cxx

icu4c: icu4c.cxx Makefile
	clang++ ${CXXFLAGS} -o icu4c -o icu4c $(shell icu-config --ldflags) icu4c.cxx

move: move.cxx Makefile
	clang++ -D_GLIBCXX_DEBUG ${CXXFLAGS} -o move move.cxx
