CFLAGS += -D_GNU_SOURCE -fPIC -I../libftp

all: libcheck

OBJS = util.o log.o list.o package.o db.o download.o trans.o \
	sync.o conf.o check.o

libcheck: $(OBJS)
	ar rcs libcheck.a $(OBJS)

clean:
	rm -f *.o *.a
