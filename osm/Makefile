download: update-streets update-house-numbers update-osmose-$(USER)

update-streets: region-streets.txt workdir/region-streets-gazdagret.txt
	./overpass-query region-streets.txt > workdir/streets.csv
	./overpass-query workdir/region-streets-gazdagret.txt > workdir/streets-gazdagret.csv

update-house-numbers: region-street-housenumbers.txt workdir/region-street-housenumbers-gazdagret.txt
	@mkdir -p workdir
	./overpass-query region-street-housenumbers.txt > workdir/street-housenumbers.csv
	./overpass-query workdir/region-street-housenumbers-gazdagret.txt > workdir/street-housenumbers-gazdagret.csv

# At the end this check found no problems, so this is not part of 'make check'.
update-postcodes:
	rm Iranyitoszam_Internet.*
	wget https://posta.hu/static/internet/download/Iranyitoszam_Internet.XLS
	libreoffice --headless --convert-to fods Iranyitoszam_Internet.XLS
	./generate-postcode-csv Iranyitoszam_Internet.fods > workdir/postcodes.csv

update-osmose-$(USER):
	@mkdir -p workdir
	curl -o workdir/osmose-$(USER).rss http://osmose.openstreetmap.fr/en/byuser/$(USER).rss

check: download
	pep8 overpass-query generate-postcode-csv qa/houseless-streets.py qa/osmose.py
	qa/osmose.py
	qa/houseless-streets.py
	#qa/houseless-streets.py -gazdagret

find-suspicious-steets:
	./find-suspicious-streets |sort -r -t $$'\t' -n -k 2

workdir/region-streets-gazdagret.txt: region-streets.txt
	@mkdir -p workdir
	sed 's/3602714372/3602713748/' region-streets.txt > workdir/region-streets-gazdagret.txt

workdir/region-street-housenumbers-gazdagret.txt: region-street-housenumbers.txt
	@mkdir -p workdir
	sed 's/3602714372/3602713748/' region-street-housenumbers.txt > workdir/region-street-housenumbers-gazdagret.txt
