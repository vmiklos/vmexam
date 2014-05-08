time sh -c "git pull -r && \
	autoconf && \
	./configure --with-epm-url=http://www.msweet.org/files/project2/epm-3.7-source.tar.gz --enable-verbose --without-stlport --enable-category-b --enable-wiki-publisher --enable-opengl --enable-dbus --enable-gstreamer --enable-bundled-dictionaries --with-package-format=archive
	make -f Makefile clean && \
	make -f Makefile " 2>&1 |tee log
