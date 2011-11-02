CFLAGS ?= -Wall -march=i686 -O2 -pipe
INSTALL = /usr/bin/install -c
DESTDIR =
prefix = /usr/local
libdir = $(prefix)/lib
includedir = $(prefix)/include

all: libfoo

foo.o: foo.c
	$(CC) -fPIC $(CFLAGS) -c foo.c
libfoo: foo.o
	$(CC) -shared -Wl,-soname,libfoo.so.0 -o libfoo.so.0.0 foo.o -lc
	ln -sf libfoo.so.0.0 libfoo.so.0
	ln -sf libfoo.so.0.0 libfoo.so

install: libfoo
	$(INSTALL) -d $(DESTDIR)$(libdir)
	$(INSTALL) libfoo.so.0.0 $(DESTDIR)$(libdir)
	ln -sf libfoo.so.0.0 $(DESTDIR)$(libdir)/libfoo.so
	$(INSTALL) -d $(DESTDIR)$(includedir)
	$(INSTALL) -m644 foo.h $(DESTDIR)$(includedir)

clean:
	rm -f *.o
	rm -f libfoo.so.0.0
