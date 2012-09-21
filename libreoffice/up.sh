withfan=""
[ "`type -p withfan`" ] && withfan="withfan"
time sh -c "git pull -r && \
	./autogen.sh && \
	make clean && \
	$withfan make && \
	make dev-install && \
	make tags && \
	make subsequentcheck && \
	git rev-parse HEAD > last-success" 2>&1 |tee log
