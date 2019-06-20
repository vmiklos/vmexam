CXXFLAGS = -g -std=c++17 -Werror -Wall -Wextra
LDFLAGS = -pthread -lPocoFoundation -lPocoJSON -lPocoNet -lPocoNetSSL
ifneq ($(ASAN),)
CXXFLAGS += -fsanitize=address
LDFLAGS += -fsanitize=address
endif
ifneq ($(UBSAN),)
CXXFLAGS += -fsanitize=undefined
LDFLAGS += -fsanitize=undefined
endif

addr-osmify: addr-osmify.cxx Makefile
	$(CXX) $(CXXFLAGS) $(LDFLAGS) -o addr-osmify addr-osmify.cxx

iwyu:
	include-what-you-use -Xiwyu --mapping_file=mapping.imp $(CXXFLAGS) -o addr-osmify addr-osmify.cxx

tidy:
	clang-tidy addr-osmify.cxx
