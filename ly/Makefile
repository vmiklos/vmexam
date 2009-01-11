SOURCES = $(wildcard *.txt)
TARGETS = $(subst .txt,.ogg,$(SOURCES))

-include .depend

.depend:
	: > .depend
	for i in $(subst .txt,,$(SOURCES)); do \
		echo -e "$$i.ogg: $$i.midi\n\
$$i.midi: $$i.pdf\n\
$$i.pdf: $$i.ly\n\
$$i.ly: $$i.html\n\
$$i.html: $$i.txt" >> .depend; \
	done

all: $(TARGETS)

%.ogg: %.midi
	timidity -o $@ -Ov $<

%.midi: %.pdf

%.pdf: %.ly
	lilypond --pdf $<

%.ly: %.html

%.html: %.txt
	asciidoc $<

clean:
	rm -f .depend *.html *.ly *.midi *.pdf *.png *.ps *.ogg
