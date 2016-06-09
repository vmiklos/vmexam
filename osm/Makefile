download: update-streets update-house-numbers update-addr-interpolation update-streets-nomaxspeed update-osmose-$(USER)

update-streets: streets.txt workdir/streets-gazdagret.txt
	./overpass-query streets.txt > workdir/streets.csv
	./overpass-query workdir/streets-gazdagret.txt > workdir/streets-gazdagret.csv

update-streets-nomaxspeed: streets-nomaxspeed.txt
	./overpass-query streets-nomaxspeed.txt > workdir/streets-nomaxspeed.csv

update-house-numbers: street-housenumbers.txt workdir/street-housenumbers-gazdagret.txt
	@mkdir -p workdir
	./overpass-query street-housenumbers.txt > workdir/street-housenumbers.csv
	./overpass-query workdir/street-housenumbers-gazdagret.txt > workdir/street-housenumbers-gazdagret.csv

update-addr-interpolation: addr-interpolation.txt workdir/street-housenumbers-gazdagret.txt
	@mkdir -p workdir
	./overpass-query addr-interpolation.txt > workdir/addr-interpolation.csv
	./overpass-query workdir/addr-interpolation-gazdagret.txt > workdir/addr-interpolation-gazdagret.csv

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
	qa/houseless-streets.py -gazdagret
	qa/interpolated-addresses.py
	#qa/interpolated-addresses.py -gazdagret
	qa/streets-nomaxspeed.py

find-suspicious-steets:
	./find-suspicious-streets |sort -r -t $$'\t' -n -k 2

workdir/streets-gazdagret.txt: streets.txt
	@mkdir -p workdir
	sed 's/3602714372/3602713748/' streets.txt > workdir/streets-gazdagret.txt

workdir/street-housenumbers-gazdagret.txt: street-housenumbers.txt
	@mkdir -p workdir
	sed 's/3602714372/3602713748/' street-housenumbers.txt > workdir/street-housenumbers-gazdagret.txt

workdir/addr-interpolation-gazdagret.txt: addr-interpolation.txt
	@mkdir -p workdir
	sed 's/3602714372/3602713748/' addr-interpolation.txt > workdir/addr-interpolation-gazdagret.txt
