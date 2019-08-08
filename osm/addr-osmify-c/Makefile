CXXFLAGS = -g -std=c11 -Werror -Wall -Wextra -I/usr/include/json-c
LDFLAGS = -lcurl -ljson-c -pthread
# Note that sanitizers can't be combined in general, but asan and ubsan is an exception.
ifneq ($(ASAN),)
CXXFLAGS += -fsanitize=address
LDFLAGS += -fsanitize=address
endif
ifneq ($(UBSAN),)
CXXFLAGS += -fsanitize=undefined
LDFLAGS += -fsanitize=undefined
endif
ifneq ($(TSAN),)
CXXFLAGS += -fsanitize=thread
LDFLAGS += -fsanitize=thread
endif

addr-osmify: addr-osmify.c Makefile
	$(CC) $(CXXFLAGS) $(LDFLAGS) -o addr-osmify addr-osmify.c

iwyu:
	include-what-you-use $(CXXFLAGS) -o addr-osmify addr-osmify.c

tidy:
	clang-tidy addr-osmify.c
