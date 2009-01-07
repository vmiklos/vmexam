cantina-band.ogg: cantina-band.midi
	timidity -o $@ -Ov $<

cantina-band.midi: cantina-band.pdf

cantina-band.pdf: cantina-band.ly
	lilypond --pdf $<

cantina-band.ly: cantina-band.html

cantina-band.html: cantina-band.txt
	asciidoc $<

clean:
	rm -f *.html *.ly *.midi *.pdf *.png *.ps *.ogg
