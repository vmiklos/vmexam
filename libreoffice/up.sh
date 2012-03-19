time sh -c 'git pull -r && \
	./autogen.sh && \
	make clean && \
	make && \
	rm -rf $(readlink install) && \
	make dev-install && \
	make subsequentcheck && \
	make tags && \
	git rev-parse HEAD > last-success' 2>&1 |tee log
