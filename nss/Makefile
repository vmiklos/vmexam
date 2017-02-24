NSS_CFLAGS := $(shell pkg-config --cflags nss)
NSS_LIBS := $(shell pkg-config --libs nss)

WORKDIR=${HOME}/scm/hg
NSS_CFLAGS_LOCAL := -I${WORKDIR}/dist/Linux4.4_x86_64_cc_glibc_PTH_64_DBG.OBJ/include -I${WORKDIR}/dist/public/nss
NSS_LIBS_LOCAL := -L${WORKDIR}/dist/Linux4.4_x86_64_cc_glibc_PTH_64_DBG.OBJ/lib -lssl3 -lsmime3 -lnss3 -lnssutil3 -lplds4 -lplc4 -lnspr4

test: test.cpp Makefile
	clang++ -g ${NSS_CFLAGS} -o test test.cpp ${NSS_LIBS}

test-local: test.cpp Makefile
	clang++ ${NSS_CFLAGS_LOCAL} -o test test.cpp ${NSS_LIBS_LOCAL}

run-local:
	LD_LIBRARY_PATH=${WORKDIR}/dist/Linux4.4_x86_64_cc_glibc_PTH_64_DBG.OBJ/lib ./test
