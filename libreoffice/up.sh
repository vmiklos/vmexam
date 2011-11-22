time sh -c 'git pull -r && \
	./autogen.sh && \
	make clean && \
	make && \
	rm -rf $(readlink install) && \
	make dev-install && \
	rm -f tags && make tags && \
	git rev-parse HEAD > last-success' 2>&1 |tee log
