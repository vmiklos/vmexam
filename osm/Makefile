download: update-streets update-house-numbers update-addr-interpolation update-streets-nomaxspeed update-osmose-$(USER)

update-streets: streets.txt workdir/streets-gazdagret.txt workdir/streets-sashegy.txt workdir/streets-nemetvolgy.txt workdir/streets-martonhegy.txt
	./overpass-query streets.txt > workdir/streets.csv
	./overpass-query workdir/streets-gazdagret.txt > workdir/streets-gazdagret.csv
	./overpass-query workdir/streets-sashegy.txt > workdir/streets-sashegy.csv
	./overpass-query workdir/streets-nemetvolgy.txt > workdir/streets-nemetvolgy.csv
	./overpass-query workdir/streets-martonhegy.txt > workdir/streets-martonhegy.csv

update-streets-nomaxspeed: streets-nomaxspeed.txt
	./overpass-query streets-nomaxspeed.txt > workdir/streets-nomaxspeed.csv

update-house-numbers: street-housenumbers.txt workdir/street-housenumbers-gazdagret.txt workdir/street-housenumbers-nemetvolgy.txt workdir/street-housenumbers-martonhegy.txt
	@mkdir -p workdir
	./overpass-query street-housenumbers.txt > workdir/street-housenumbers.csv
	./overpass-query workdir/street-housenumbers-gazdagret.txt > workdir/street-housenumbers-gazdagret.csv
	./overpass-query workdir/street-housenumbers-sashegy.txt > workdir/street-housenumbers-sashegy.csv
	./overpass-query workdir/street-housenumbers-nemetvolgy.txt > workdir/street-housenumbers-nemetvolgy.csv
	./overpass-query workdir/street-housenumbers-martonhegy.txt > workdir/street-housenumbers-martonhegy.csv

update-addr-interpolation: addr-interpolation.txt workdir/street-housenumbers-gazdagret.txt workdir/street-housenumbers-sashegy.txt
	@mkdir -p workdir
	./overpass-query addr-interpolation.txt > workdir/addr-interpolation.csv
	./overpass-query workdir/addr-interpolation-gazdagret.txt > workdir/addr-interpolation-gazdagret.csv
	./overpass-query workdir/addr-interpolation-sashegy.txt > workdir/addr-interpolation-sashegy.csv
	./overpass-query workdir/addr-interpolation-nemetvolgy.txt > workdir/addr-interpolation-nemetvolgy.csv

# At the end this check found no problems, so this is not part of 'make check'.
update-postcodes:
	rm Iranyitoszam_Internet.*
	wget https://posta.hu/static/internet/download/Iranyitoszam_Internet.XLS
	libreoffice --headless --convert-to fods Iranyitoszam_Internet.XLS
	./generate-postcode-csv Iranyitoszam_Internet.fods > workdir/postcodes.csv

update-osmose-$(USER):
	@mkdir -p workdir
	curl -o workdir/osmose-$(USER).rss http://osmose.openstreetmap.fr/en/byuser/$(USER).rss

check-style:
	pep8 overpass-query generate-postcode-csv get-reference-housenumbers qa/suspicious-streets.py qa/houseless-streets.py qa/osmose.py

check: check-style download
	qa/osmose.py
	qa/houseless-streets.py
	qa/houseless-streets.py -gazdagret
	qa/houseless-streets.py -sashegy
	qa/houseless-streets.py -nemetvolgy
	qa/houseless-streets.py -martonhegy
	qa/interpolated-addresses.py
	qa/interpolated-addresses.py -gazdagret
	qa/interpolated-addresses.py -sashegy
	qa/interpolated-addresses.py -nemetvolgy
	qa/streets-nomaxspeed.py
	qa/suspicious-streets.py
	#qa/suspicious-streets.py -gazdagret

workdir/streets-gazdagret.txt: streets.txt
	@mkdir -p workdir
	sed 's/3602714372/3602713748/' streets.txt > workdir/streets-gazdagret.txt

workdir/street-housenumbers-gazdagret.txt: street-housenumbers.txt
	@mkdir -p workdir
	sed 's/3602714372/3602713748/' street-housenumbers.txt > workdir/street-housenumbers-gazdagret.txt

workdir/addr-interpolation-gazdagret.txt: addr-interpolation.txt
	@mkdir -p workdir
	sed 's/3602714372/3602713748/' addr-interpolation.txt > workdir/addr-interpolation-gazdagret.txt

workdir/addr-interpolation-sashegy.txt: addr-interpolation.txt
	@mkdir -p workdir
	sed 's/3602714372/3602714524/' addr-interpolation.txt > workdir/addr-interpolation-sashegy.txt

workdir/addr-interpolation-nemetvolgy.txt: addr-interpolation.txt
	@mkdir -p workdir
	sed 's/3602714372/3602806940/' addr-interpolation.txt > workdir/addr-interpolation-nemetvolgy.txt

workdir/streets-sashegy.txt: streets.txt
	@mkdir -p workdir
	sed 's/3602714372/3602714524/' streets.txt > workdir/streets-sashegy.txt

workdir/streets-nemetvolgy.txt: streets.txt
	@mkdir -p workdir
	sed 's/3602714372/3602806940/' streets.txt > workdir/streets-nemetvolgy.txt

workdir/streets-martonhegy.txt: streets.txt
	@mkdir -p workdir
	sed 's/3602714372/3602787965/' streets.txt > workdir/streets-martonhegy.txt

workdir/street-housenumbers-sashegy.txt: street-housenumbers.txt
	@mkdir -p workdir
	sed 's/3602714372/3602714524/' street-housenumbers.txt > workdir/street-housenumbers-sashegy.txt

workdir/street-housenumbers-nemetvolgy.txt: street-housenumbers.txt
	@mkdir -p workdir
	sed 's/3602714372/3602806940/' street-housenumbers.txt > workdir/street-housenumbers-nemetvolgy.txt

workdir/street-housenumbers-martonhegy.txt: street-housenumbers.txt
	@mkdir -p workdir
	sed 's/3602714372/3602787965/' street-housenumbers.txt > workdir/street-housenumbers-martonhegy.txt
