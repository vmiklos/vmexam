time sh -c "git pull -r && \
	./autogen.sh && \
	make clean && \
	make build-nocheck &&
	sh ~/git/vmexam/libreoffice/daily.sh && \
	make tags && \
	(cd instdir && ln -s $HOME/.config/libreofficedev/4/user) && \
	make check &&
	style-check-files" 2>&1 |tee log
