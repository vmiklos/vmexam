all: test.pdf

test.pdf: base.dot rebase.dot merge.dot base.eps rebase.eps merge.eps

%.eps: %.dot
	dot -Teps -o $@ $<

%.pdf: %.txt
	a2beamer $<
