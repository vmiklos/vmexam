ifeq ($(shell uname),Darwin)
OBJC_LIBS = -framework Foundation
else
OBJC_FLAGS = $(shell gnustep-config --objc-flags)
OBJC_LIBS = $(shell gnustep-config --base-libs)
endif

main: main.m Makefile
	clang $(OBJC_FLAGS) $(OBJC_LIBS) -o main main.m
