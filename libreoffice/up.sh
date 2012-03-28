time sh -c 'git pull -r && \
	./autogen.sh && \
	make clean && \
	make && \
	make dev-install && \
	make tags && \
	(for i in oox writerfilter sw; do cd $i; make clean; make -sr dbglevel=2 -j4; cd -; done) && \
	make subsequentcheck && \
	git rev-parse HEAD > last-success' 2>&1 |tee log
