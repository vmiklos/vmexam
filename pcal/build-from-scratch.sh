rm -rf scratch
mkdir scratch
cd scratch
src=""
for i in $(seq -w 1 12)
do
	pcal -f ../calendar_hu.txt $i 2017 > cal$i.ps
	ps2pdf cal$i.ps
	convert ../pic/$i.jpg img$i.pdf
	src="$src img$i.pdf cal$i.pdf"
done
pdftk $src cat output withpic.pdf
# print to ps; ideally pdftops would do the same, but actually it does not (and
# gs is horribly broken at pdf2ps)
okular withpic.pdf
pstops '2:0L@0.6(20cm,2cm)+1L@0.6(20cm,15cm)' withpic.ps multipage.ps
ps2pdf multipage.ps
pdftk multipage.pdf cat 1-endsouth output rotated.pdf
