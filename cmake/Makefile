all:
	@mkdir -p workdir
ifneq ($(OS),Windows_NT)
	cd workdir && cmake -DCMAKE_INSTALL_PREFIX:PATH=$(PWD)/instdir -DCMAKE_BUILD_TYPE=Debug ..
else
	cd workdir && cmake -DCMAKE_INSTALL_PREFIX:PATH=$(shell cygpath -m $(PWD))/instdir -DCMAKE_BUILD_TYPE=Debug ..
endif
	cd workdir && cmake --build . --target install

clean:
	rm -rf instdir
	rm -rf workdir

run:
	instdir/bin/hello
