time sh -c "git pull -r && \
	autoconf && \
	./configure --with-epm-url=http://www.msweet.org/files/project2/epm-3.7-source.tar.gz --enable-verbose --enable-category-b --with-package-format=archive
	make -f Makefile clean && \
	make -f Makefile " 2>&1 |tee log
