CFLAGS ?= -march=i686 -O2 -pipe
CFLAGS += $(shell mysql_config --cflags)
LDFLAGS = $(shell mysql_config --libs)

all: select
