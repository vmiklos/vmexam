ifeq ($(shell uname),Darwin)
OBJC_LIBS = -framework Foundation
else
OBJC_FLAGS = $(shell gnustep-config --objc-flags)
OBJC_LIBS = $(shell gnustep-config --base-libs)
endif

main: main.m Makefile
	clang -fobjc-arc -g -Wall -Wextra -Werror $(OBJC_FLAGS) $(OBJC_LIBS) -o main main.m

check:
	clang -fobjc-arc --analyze main.m
