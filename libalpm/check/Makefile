CFLAGS += -D_GNU_SOURCE -Ilibftp -Ilibcheck
LDFLAGS += -Llibcheck -lcheck -lalpm -Llibftp -lftp

main: libftp.a libcheck.a
	$(CC) $(CFLAGS) -o main main.c $(LDFLAGS)

libftp.a:
	$(MAKE) -C libftp all

libcheck.a:
	$(MAKE) -C libcheck all

clean:
	rm -f main *.o
	$(MAKE) -C libftp clean
	$(MAKE) -C libcheck clean
