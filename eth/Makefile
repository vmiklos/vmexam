CFLAGS += -I/usr/include/pci -ggdb
LDFLAGS += -lpci -lz
all: driver pci

pci: pci.c
	$(CC) $(CFLAGS) $^ -o $@ $(LDFLAGS)
