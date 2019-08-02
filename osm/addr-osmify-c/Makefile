CXXFLAGS = -g -std=c11 -Werror -Wall -Wextra -I/usr/include/json-c
LDFLAGS = -lcurl -ljson-c
ifneq ($(ASAN),)
CXXFLAGS += -fsanitize=address
LDFLAGS += -fsanitize=address
endif
ifneq ($(UBSAN),)
CXXFLAGS += -fsanitize=undefined
LDFLAGS += -fsanitize=undefined
endif

addr-osmify: addr-osmify.c Makefile
	$(CC) $(CXXFLAGS) $(LDFLAGS) -o addr-osmify addr-osmify.c

iwyu:
	include-what-you-use $(CXXFLAGS) -o addr-osmify addr-osmify.c

# TODO tidy
