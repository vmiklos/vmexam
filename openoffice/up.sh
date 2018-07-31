# git clone git://git.apache.org/openoffice.git
# tested in a Ubuntu 14.04 chroot, build breaks with a non-ancient toolchain
# last successfull commit: 3003f94b10b21648b953df2e71d66680e3605f06 (Added graphics for Norwegian, 2018-07-29)
time sh -c "git pull -r && \
	autoconf && \
	./configure --disable-epm --with-dmake-url=https://sourceforge.net/projects/oooextras.mirror/files/dmake-4.12.tar.bz2 && \
	make -f Makefile clean && \
	rm -rf install && \
	make -f Makefile && \
	(. ./*Env.Set.sh && cd instsetoo_native/util; LOCALINSTALLDIR=$(pwd)/install dmake openoffice_en-US PKGFORMAT=installed)" 2>&1 |tee log
