time sh -c "git pull -r && \
	./autogen.sh && \
	echo 'export MAKEFLAGS=$MAKEFLAGS' >> config_host.mk && \
	make clean && \
	make && \
	make dev-install && \
	make tags && \
	make subsequentcheck && \
	git rev-parse HEAD > last-success" 2>&1 |tee log
