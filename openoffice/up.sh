time sh -c "git pull -r && \
	autoconf && \
	./configure --disable-epm --enable-category-b --with-package-format=archive
	make -f Makefile clean && \
	rm -rf install && \
	make -f Makefile && \
	(. ./*Env.Set.sh && cd instsetoo_native/util; LOCALINSTALLDIR=$(pwd)/../../install dmake openoffice_en-US PKGFORMAT=installed)" 2>&1 |tee log
