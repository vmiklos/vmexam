time sh -c "git pull -r && \
	./autogen.sh && \
	make clean && \
	make &&
	sh ~/git/vmexam/libreoffice/daily.sh && \
	make tags && \
	(cd instdir && mv user user.orig; ln -s $HOME/.config/libreofficedev/4/user) && \
	make subsequentcheck" 2>&1 |tee log
