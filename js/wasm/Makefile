# Resources:
# - https://github.com/PetterS/clang-wasm
# - https://aransentin.github.io/cwasm/

CXXFLAGS = -Wall -Wextra -Werror -std=c++17 -O2 -fvisibility=hidden --target=wasm32

LDFLAGS = --no-entry --strip-all --export-dynamic

library.wasm: library.o Makefile
	${HOME}/git/llvm/instdir/bin/wasm-ld ${LDFLAGS} -o $@ $<


library.o: library.cpp Makefile
	${HOME}/git/llvm/instdir/bin/clang++ ${CXXFLAGS} -c -o $@ $<
