CFLAGS  += -DFTPLIB_DEFMODE=FTPLIB_PORT -D_REENTRANT

all: libftp

ftplib.o:
	$(CC) -fPIC $(CFLAGS) -c ftplib.c

libftp: ftplib.o
	ar rcs libftp.a ftplib.o

clean:
	rm -f *.o *.a
