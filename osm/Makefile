download: update-streets update-house-numbers update-osmose-$(USER)

update-streets:
	@mkdir -p workdir
	./overpass-query region-streets.txt > workdir/streets.csv

update-house-numbers:
	@mkdir -p workdir
	./overpass-query region-street-housenumbers.txt > workdir/street-housenumbers.csv

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

find-suspicious-steets:
	./find-suspicious-streets |sort -r -t $$'\t' -n -k 2
