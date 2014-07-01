for i in *.pdf
do
	name="$(basename "$i" .pdf)"
	echo "$name"
	if [ ! -e "${name}-1.png" ]; then
		pdftocairo -png $i
	fi
	cd rt
	if [ ! -e "rt/${name}-1.png" ]; then
		pdftocairo -png $i
	fi
	cd ..
	montage "${name}-1.png" "rt/${name}-1.png" orig-label.png rt-label.png -geometry +2+2 "out/Comparison Screenshot - ${name}.png"
done
