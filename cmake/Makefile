all:
	@mkdir -p workdir
	cd workdir && cmake -DCMAKE_INSTALL_PREFIX:PATH=$(PWD)/instdir -DCMAKE_BUILD_TYPE=Debug .. && $(MAKE) $(if $(verbose)$(VERBOSE),VERBOSE=1) install

clean:
	rm -rf instdir
	rm -rf workdir

run:
	instdir/bin/hello
