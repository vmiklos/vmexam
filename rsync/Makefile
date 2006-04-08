CFLAGS ?= -Wall -march=i686 -O2 -pipe
LDFLAGS = -lrsync -lpopt
TESTURL = rsync://localhost/ftp/pub/frugalware/frugalware-current/AUTHORS

foo:
	$(CC) $(CFLAGS) -o foo foo.c $(LDFLAGS)

static:
	$(CC) $(CFLAGS) -o foo.static foo.c -static $(LDFLAGS)

clean:
	rm -f foo foo.static

test: foo
	./foo $(TESTURL) ./

test-static: static
	./foo.static $(TESTURL) ./
