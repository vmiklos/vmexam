check:
	pycodestyle ./diff-backend
	./diff-backend mscrypto-status.txt mscng-status.txt

transform: Makefile transform.cxx
	clang++ -o transform transform.cxx $(shell pkg-config --cflags xmlsec1-nss) $(shell pkg-config --libs xmlsec1-nss)
