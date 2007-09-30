doc: HEADER.html

HEADER.html: README
	ln -s README HEADER.txt
	asciidoc -a toc -a numbered -a sectids HEADER.txt
	rm HEADER.txt

dist:
	git log --no-merges . |git name-rev --tags --stdin >ChangeLog
	tar czf pyrssi.tar.gz .htaccess pyrssi.py config.py README socket-interface.pl ChangeLog
