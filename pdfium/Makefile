PDFIUM_CFLAGS := -I${HOME}/git/pdfium/pdfium/public -I${HOME}/git/pdfium/pdfium/ -I/usr/include/freetype2
P := ${HOME}/git/pdfium/pdfium/workdir
LIBCXX = $(wildcard ${P}/obj/buildtools/third_party/libc++/libc++/*.o) $(wildcard ${P}/obj/buildtools/third_party/libc++abi/libc++abi/*.o)
PDFIUM_LIBS := -Wl,--start-group \
	${P}/obj/libpdfium.a \
	${LIBCXX} \
	-Wl,--end-group  -ldl -lrt -lfreetype -pthread

test: test.o Makefile
	clang++ -std=c++11 -g -o test test.o ${PDFIUM_LIBS}

test.o: test.cxx Makefile
	clang++ -std=c++11 -g -c -o test.o ${PDFIUM_CFLAGS} test.cxx
