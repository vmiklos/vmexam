LDFLAGS = -lgsoap

OBJS = googleapi.o soapClient.o soapC.o

googleapi: $(OBJS)
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS)

clean:
	rm -f googleapi $(OBJS)
