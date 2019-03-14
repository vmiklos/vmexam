CXXFLAGS = -g -std=c++17 -Werror -Wall -Wextra
LDFLAGS = -lPocoFoundation -lPocoJSON -lPocoNet -lPocoNetSSL

addr-osmify: addr-osmify.cxx Makefile
	$(CXX) $(CXXFLAGS) $(LDFLAGS) -o addr-osmify addr-osmify.cxx

iwyu:
	include-what-you-use -Xiwyu --mapping_file=mapping.imp $(CXXFLAGS) -o addr-osmify addr-osmify.cxx

tidy:
	clang-tidy addr-osmify.cxx
