download: update-streets update-house-numbers

update-streets:
	@mkdir -p workdir
	./overpass-query region-streets.txt > workdir/streets.csv

update-house-numbers:
	@mkdir -p workdir
	./overpass-query region-street-housenumbers.txt > workdir/street-housenumbers.csv

check:
	pep8 find-houseless-streets overpass-query qa/houseless-streets.py
