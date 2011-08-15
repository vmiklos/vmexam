time sh -c 'git pull -r && \
	./autogen.sh && \
	make clean && \
	make && \
	rm -rf $(readlink install) && \
	make dev-install && \
	git rev-parse HEAD > last-success'
