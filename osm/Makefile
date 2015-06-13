update-house-numbers:
	@mkdir -p out
	./overpass-query region-street-housenumbers.txt > out/street-housenumbers.csv

check:
	pep8 --ignore=E501 find-houseless-streets overpass-query
