download: update-streets update-house-numbers update-osmose-$(USER)

update-streets:
	@mkdir -p workdir
	./overpass-query region-streets.txt > workdir/streets.csv

update-house-numbers:
	@mkdir -p workdir
	./overpass-query region-street-housenumbers.txt > workdir/street-housenumbers.csv

update-osmose-$(USER):
	@mkdir -p workdir
	curl -o workdir/osmose-$(USER).rss http://osmose.openstreetmap.fr/en/byuser/$(USER).rss

check: download
	pep8 overpass-query qa/houseless-streets.py qa/osmose.py
	qa/osmose.py
