# git clone git://git.apache.org/openoffice.git
time sh -c "git pull -r && \
	autoconf && \
	./configure --disable-epm
	make -f Makefile clean && \
	rm -rf install && \
	make -f Makefile && \
	(. ./*Env.Set.sh && cd instsetoo_native/util; LOCALINSTALLDIR=$(pwd)/install dmake openoffice_en-US PKGFORMAT=installed)" 2>&1 |tee log
