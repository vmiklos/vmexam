download: update-osmose-$(USER)

update-streets: streets.txt
	../../osm-gimmisn/overpass_query.py streets.txt > workdir/streets-sasad.csv

update-streets-nomaxspeed: streets-nomaxspeed.txt
	../../osm-gimmisn/overpass_query.py streets-nomaxspeed.txt > workdir/streets-nomaxspeed.csv

update-house-numbers: street-housenumbers.txt
	@mkdir -p workdir
	../../osm-gimmisn/overpass_query.py street-housenumbers.txt > workdir/street-housenumbers-sasad.csv

update-addr-interpolation: addr-interpolation.txt
	@mkdir -p workdir
	../../osm-gimmisn/overpass_query.py addr-interpolation.txt > workdir/addr-interpolation.csv

# At the end this check found no problems, so this is not part of 'make check'.
update-postcodes:
	rm -f Iranyitoszam-Internet_uj.*
	wget https://www.posta.hu/static/internet/download/Iranyitoszam-Internet_uj.xlsx
	libreoffice --headless --convert-to fods Iranyitoszam-Internet_uj.xlsx
	./generate-postcode-csv Iranyitoszam-Internet_uj.fods > workdir/postcodes.csv

update-osmose-$(USER):
	@mkdir -p workdir
	curl -L -o workdir/osmose-$(USER).rss http://osmose.openstreetmap.fr/en/byuser/$(USER).rss

PYFILES = generate-postcode-csv qa/osmose.py

check-style:
	flake8 $(PYFILES)

check: check-style download
	qa/osmose.py
	#qa/interpolated-addresses.py
	#qa/streets-nomaxspeed.py
