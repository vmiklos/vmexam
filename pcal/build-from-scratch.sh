rm -rf scratch
mkdir scratch
cd scratch
src=""
for i in $(seq -w 1 12)
do
	# We typically do a calendar for the next year.
	../pcal -f ../calendar_hu.txt $i $(($(date +%Y)+1)) > cal$i.ps
	pnmtopsopts="-width 11.7 -height 8.3"
	convert ../pic/$i.jpg img$i.pnm
	pnmtops -noturn $pnmtopsopts img$i.pnm >img$i.ps
	rm img$i.pnm 
	gsopts="-dDEVICEWIDTHPOINTS=842 -dDEVICEHEIGHTPOINTS=595"
	gs -q -dNOPAUSE -dBATCH -sDEVICE=pdfwrite $gsopts -sOutputFile="img$i.pdf" img$i.ps
	gs -q -dNOPAUSE -dBATCH -sDEVICE=pdfwrite $gsopts -sOutputFile="cal$i.pdf" cal$i.ps
	src="$src img$i.pdf cal$i.pdf"
done
pdftk $src cat output withpic.pdf
# print to ps; ideally pdftops would do the same, but actually it does not (and
# gs is horribly broken at pdf2ps)
okular withpic.pdf
pstops '2:0L@0.6(20cm,2cm)+1L@0.6(20cm,15cm)' withpic.ps multipage.ps
ps2pdf multipage.ps
pdftk multipage.pdf cat 1-endsouth output rotated.pdf
