LDFLAGS = -larchive -lbz2 -lz

all: l e
clean:
	rm -f *.o list extract
LOBJS = list.o
l: $(LOBJS)
	$(CC) $(CFLAGS) -o list $(LOBJS) $(LDFLAGS)
EOBJS = extract.o
e: $(EOBJS)
	$(CC) $(CFLAGS) -o extract $(EOBJS) $(LDFLAGS)
