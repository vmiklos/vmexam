for i in *.pdf
do
	name="$(basename "$i" .pdf)"
	echo "$name"
	montage "${name}.pdf-1.png" "rt/${name}.pdf-1.png" orig-label.png rt-label.png -geometry +2+2 "out/Comparison Screenshot - ${name}.png"
done
