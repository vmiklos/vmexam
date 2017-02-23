NSS_CFLAGS := $(shell pkg-config --cflags nss)
NSS_LIBS := $(shell pkg-config --libs nss)

test: test.cpp Makefile
	clang++ -g ${NSS_CFLAGS} -o test test.cpp ${NSS_LIBS}
