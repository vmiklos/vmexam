all: test.pdf

%.pdf: %.txt
	a2beamer $<

%.1: %.mp
	mpost $<

%.eps: %.1
	ln -sf $< $@

.SECONDARY:

# this could be auto-generated
test.txt: test.eps
