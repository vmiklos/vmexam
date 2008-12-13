all: test.png

%.png: %.1001.ppm
	convert $< $@

%.1001.ppm: %.1
	pstopnm -xborder 0 -yborder 0 -portrait $<

%.1: %.mp
	mpost $<
