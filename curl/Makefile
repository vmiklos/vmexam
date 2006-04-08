CFLAGS ?= -march=i686 -O2 -pipe
LDFLAGS = $(shell curl-config --libs)

all: dl dl_pure up up_pure post

dl: util.o dl.o
	$(CC) $(LDFLAGS) util.o dl.o -o $@

dl_pure: util.o dl_pure.o
	$(CC) $(LDFLAGS) util.o dl_pure.o -o $@

up: util.o up.o
	$(CC) $(LDFLAGS) util.o up.o -o $@

up_pure: util.o up_pure.o
	$(CC) $(LDFLAGS) util.o up_pure.o -o $@

post: util.o post.o
	$(CC) $(LDFLAGS) util.o post.o -o $@

clean:
	rm -f *.o

distclean: clean
	rm -rf dl dl_pure up up_pure post
