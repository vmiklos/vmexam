json: json.cc Makefile
	g++ -std=c++11 -o json json.cc

json-array-read: json-array-read.cc Makefile
	g++ -std=c++11 -o json-array-read json-array-read.cc

poco: poco.cc Makefile
	g++ -std=c++11 -o poco poco.cc -lPocoFoundation -lPocoJSON

crtp: crtp.cc Makefile
	g++ -std=c++11 -o crtp crtp.cc

python-filter: python-filter.cxx Makefile
	g++ -std=c++11 -o python-filter python-filter.cxx

python-map: python-map.cxx Makefile
	g++ -std=c++11 -o python-map python-map.cxx

python-random-choice: python-random-choice.cxx Makefile
	g++ -std=c++11 -o python-random-choice python-random-choice.cxx

scope-guard: scope-guard.cxx Makefile
	g++ -std=c++11 -o scope-guard scope-guard.cxx

async: async.cxx Makefile
	g++ -std=c++11 -pthread -g -o async async.cxx

asan: asan.cxx Makefile
	clang++ -std=c++11 -o asan -g -O0 -fsanitize=address asan.cxx
