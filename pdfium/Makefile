PDFIUM_CFLAGS := -I${HOME}/git/pdfium/pdfium/public -I${HOME}/git/pdfium/pdfium/ -I/usr/include/freetype2
P := ${HOME}/git/pdfium/pdfium/workdir
PDFIUM_LIBS := -Wl,--start-group \
	${P}/obj/libpdfium.a \
	-Wl,--end-group  -ldl -lrt -lfreetype -pthread

test: test.o Makefile
	clang++ -std=c++11 -o test test.o ${PDFIUM_LIBS}

test.o: test.cxx Makefile
	clang++ -std=c++11 -c -o test.o ${PDFIUM_CFLAGS} test.cxx
