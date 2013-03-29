time sh -c "git pull -r && \
	./autogen.sh && \
	make clean && \
	make dev-install && \
	sh ~/git/vmexam/libreoffice/daily.sh && \
	make tags && \
	make subsequentcheck" 2>&1 |tee log
