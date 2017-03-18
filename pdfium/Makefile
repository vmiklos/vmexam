PDFIUM_CFLAGS := -I${HOME}/git/pdfium/pdfium/public
P := ${HOME}/git/pdfium/pdfium/workdir
PDFIUM_LIBS := -Wl,--start-group \
	${P}/obj/libpdfium.a \
	${P}/obj/libtest_support.a \
	${P}/obj/third_party/libfx_freetype.a \
	${P}/obj/third_party/libfx_lpng.a \
	${P}/obj/libfxcrt.a \
	${P}/obj/libfdrm.a \
	${P}/obj/libformfiller.a \
	${P}/obj/libfpdfapi.a \
	${P}/obj/libfpdfdoc.a \
	${P}/obj/libfpdftext.a \
	${P}/obj/libfxcodec.a \
	${P}/obj/third_party/libfx_lcms2.a \
	${P}/obj/third_party/libfx_libopenjpeg.a \
	${P}/obj/third_party/libfx_zlib.a \
	${P}/obj/third_party/libjpeg.a \
	${P}/obj/libfxedit.a \
	${P}/obj/libfxge.a \
	${P}/obj/third_party/libfx_agg.a \
	${P}/obj/libjavascript.a \
	${P}/obj/libpdfwindow.a \
	${P}/obj/third_party/libbigint.a \
	${P}/obj/testing/gmock/libgmock.a \
	${P}/obj/testing/gtest/libgtest.a \
	-Wl,--end-group  -ldl -lrt -lfreetype -pthread

test: test.o Makefile
	clang++ -std=c++11 -o test test.o ${PDFIUM_LIBS}

test.o: test.cxx Makefile
	clang++ -std=c++11 -c -o test.o ${PDFIUM_CFLAGS} test.cxx
