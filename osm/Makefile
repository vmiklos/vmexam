download: update-streets update-house-numbers update-addr-interpolation update-streets-nomaxspeed update-osmose-$(USER)

update-streets: streets.txt workdir/streets-gazdagret.txt workdir/streets-sashegy.txt workdir/streets-nemetvolgy.txt workdir/streets-martonhegy.txt workdir/streets-ormezo.txt workdir/streets-krisztinavaros.txt
	./overpass-query streets.txt > workdir/streets.csv
	./overpass-query workdir/streets-gazdagret.txt > workdir/streets-gazdagret.csv
	./overpass-query workdir/streets-sashegy.txt > workdir/streets-sashegy.csv
	./overpass-query workdir/streets-nemetvolgy.txt > workdir/streets-nemetvolgy.csv
	./overpass-query workdir/streets-martonhegy.txt > workdir/streets-martonhegy.csv
	./overpass-query workdir/streets-ormezo.txt > workdir/streets-ormezo.csv
	./overpass-query workdir/streets-farkasvolgy.txt > workdir/streets-farkasvolgy.csv
	./overpass-query workdir/streets-magasut.txt > workdir/streets-magasut.csv
	./overpass-query workdir/streets-krisztinavaros.txt > workdir/streets-krisztinavaros.csv
	./overpass-query workdir/streets-balatonboglar.txt > workdir/streets-balatonboglar.csv
	./overpass-query workdir/streets-szigetmonostor.txt > workdir/streets-szigetmonostor.csv

update-streets-nomaxspeed: streets-nomaxspeed.txt
	./overpass-query streets-nomaxspeed.txt > workdir/streets-nomaxspeed.csv

update-house-numbers: street-housenumbers.txt workdir/street-housenumbers-gazdagret.txt workdir/street-housenumbers-sashegy.txt workdir/street-housenumbers-nemetvolgy.txt workdir/street-housenumbers-martonhegy.txt workdir/street-housenumbers-ormezo.txt workdir/street-housenumbers-krisztinavaros.txt
	@mkdir -p workdir
	./overpass-query street-housenumbers.txt > workdir/street-housenumbers.csv
	./overpass-query workdir/street-housenumbers-gazdagret.txt > workdir/street-housenumbers-gazdagret.csv
	./overpass-query workdir/street-housenumbers-sashegy.txt > workdir/street-housenumbers-sashegy.csv
	./overpass-query workdir/street-housenumbers-nemetvolgy.txt > workdir/street-housenumbers-nemetvolgy.csv
	./overpass-query workdir/street-housenumbers-martonhegy.txt > workdir/street-housenumbers-martonhegy.csv
	./overpass-query workdir/street-housenumbers-ormezo.txt > workdir/street-housenumbers-ormezo.csv
	./overpass-query workdir/street-housenumbers-magasut.txt > workdir/street-housenumbers-magasut.csv
	./overpass-query workdir/street-housenumbers-balatonboglar.txt > workdir/street-housenumbers-balatonboglar.csv
	./overpass-query workdir/street-housenumbers-szigetmonostor.txt > workdir/street-housenumbers-szigetmonostor.csv
	./overpass-query workdir/street-housenumbers-krisztinavaros.txt > workdir/street-housenumbers-krisztinavaros.csv
	./overpass-query workdir/street-housenumbers-lagymanyos.txt > workdir/street-housenumbers-lagymanyos.csv

update-addr-interpolation: addr-interpolation.txt workdir/street-housenumbers-gazdagret.txt workdir/street-housenumbers-sashegy.txt
	@mkdir -p workdir
	./overpass-query addr-interpolation.txt > workdir/addr-interpolation.csv
	./overpass-query workdir/addr-interpolation-gazdagret.txt > workdir/addr-interpolation-gazdagret.csv
	./overpass-query workdir/addr-interpolation-sashegy.txt > workdir/addr-interpolation-sashegy.csv
	./overpass-query workdir/addr-interpolation-nemetvolgy.txt > workdir/addr-interpolation-nemetvolgy.csv
	./overpass-query workdir/addr-interpolation-martonhegy.txt > workdir/addr-interpolation-martonhegy.csv
	./overpass-query workdir/addr-interpolation-ormezo.txt > workdir/addr-interpolation-ormezo.csv

# At the end this check found no problems, so this is not part of 'make check'.
update-postcodes:
	rm -f Iranyitoszam-Internet_uj.*
	wget https://www.posta.hu/static/internet/download/Iranyitoszam-Internet_uj.xlsx
	libreoffice --headless --convert-to fods Iranyitoszam-Internet_uj.xlsx
	./generate-postcode-csv Iranyitoszam-Internet_uj.fods > workdir/postcodes.csv

update-osmose-$(USER):
	@mkdir -p workdir
	curl -o workdir/osmose-$(USER).rss http://osmose.openstreetmap.fr/en/byuser/$(USER).rss

check-style:
	pycodestyle overpass-query generate-postcode-csv get-reference-housenumbers qa/suspicious-streets.py qa/houseless-streets.py qa/osmose.py addr-osmify

check: check-style download
	qa/osmose.py
	qa/houseless-streets.py
	qa/houseless-streets.py -gazdagret
	qa/houseless-streets.py -sashegy
	qa/houseless-streets.py -nemetvolgy
	qa/houseless-streets.py -martonhegy
	qa/houseless-streets.py -ormezo
	qa/houseless-streets.py -farkasvolgy
	#qa/houseless-streets.py -magasut
	#qa/houseless-streets.py -balatonboglar
	#qa/houseless-streets.py -szigetmonostor
	#qa/houseless-streets.py -krisztinavaros
	qa/interpolated-addresses.py
	qa/interpolated-addresses.py -gazdagret
	qa/interpolated-addresses.py -sashegy
	qa/interpolated-addresses.py -nemetvolgy
	qa/interpolated-addresses.py -martonhegy
	qa/interpolated-addresses.py -ormezo
	qa/streets-nomaxspeed.py
	qa/suspicious-streets.py
	qa/suspicious-streets.py -gazdagret
	qa/suspicious-streets.py -sashegy
	qa/suspicious-streets.py -nemetvolgy
	qa/suspicious-streets.py -ormezo
	qa/suspicious-streets.py -farkasvolgy

workdir/streets-balatonboglar.txt: streets.txt
	@mkdir -p workdir
	sed 's/3602714372/3601367832/' streets.txt > workdir/streets-balatonboglar.txt

workdir/streets-szigetmonostor.txt: streets.txt
	@mkdir -p workdir
	sed 's/3602714372/3600216157/' streets.txt > workdir/streets-szigetmonostor.txt

workdir/streets-gazdagret.txt: streets.txt
	@mkdir -p workdir
	sed 's/3602714372/3602713748/' streets.txt > workdir/streets-gazdagret.txt

workdir/street-housenumbers-gazdagret.txt: street-housenumbers.txt
	@mkdir -p workdir
	sed 's/3602714372/3602713748/' street-housenumbers.txt > workdir/street-housenumbers-gazdagret.txt

workdir/street-housenumbers-balatonboglar.txt: street-housenumbers.txt
	@mkdir -p workdir
	sed 's/3602714372/3601367832/' street-housenumbers.txt > workdir/street-housenumbers-balatonboglar.txt

workdir/street-housenumbers-szigetmonostor.txt: street-housenumbers.txt
	@mkdir -p workdir
	sed 's/3602714372/3600216157/' street-housenumbers.txt > workdir/street-housenumbers-szigetmonostor.txt

workdir/addr-interpolation-gazdagret.txt: addr-interpolation.txt
	@mkdir -p workdir
	sed 's/3602714372/3602713748/' addr-interpolation.txt > workdir/addr-interpolation-gazdagret.txt

workdir/addr-interpolation-sashegy.txt: addr-interpolation.txt
	@mkdir -p workdir
	sed 's/3602714372/3602714524/' addr-interpolation.txt > workdir/addr-interpolation-sashegy.txt

workdir/addr-interpolation-nemetvolgy.txt: addr-interpolation.txt
	@mkdir -p workdir
	sed 's/3602714372/3602806940/' addr-interpolation.txt > workdir/addr-interpolation-nemetvolgy.txt

workdir/addr-interpolation-ormezo.txt: addr-interpolation.txt
	@mkdir -p workdir
	sed 's/3602714372/3602713749/' addr-interpolation.txt > workdir/addr-interpolation-ormezo.txt

workdir/addr-interpolation-martonhegy.txt: addr-interpolation.txt
	@mkdir -p workdir
	sed 's/3602714372/3602787965/' addr-interpolation.txt > workdir/addr-interpolation-martonhegy.txt

workdir/streets-sashegy.txt: streets.txt
	@mkdir -p workdir
	sed 's/3602714372/3602714524/' streets.txt > workdir/streets-sashegy.txt

workdir/streets-nemetvolgy.txt: streets.txt
	@mkdir -p workdir
	sed 's/3602714372/3602806940/' streets.txt > workdir/streets-nemetvolgy.txt

workdir/streets-martonhegy.txt: streets.txt
	@mkdir -p workdir
	sed 's/3602714372/3602787965/' streets.txt > workdir/streets-martonhegy.txt

workdir/streets-ormezo.txt: streets.txt
	@mkdir -p workdir
	sed 's/3602714372/3602713749/' streets.txt > workdir/streets-ormezo.txt

workdir/streets-farkasvolgy.txt: streets.txt
	@mkdir -p workdir
	sed 's/3602714372/3602788020/' streets.txt > workdir/streets-farkasvolgy.txt

workdir/streets-magasut.txt: streets.txt
	@mkdir -p workdir
	sed 's/3602714372/3602805999/' streets.txt > workdir/streets-magasut.txt

workdir/streets-krisztinavaros.txt: streets.txt
	@mkdir -p workdir
	sed 's/3602714372/3602786310/' streets.txt > workdir/streets-krisztinavaros.txt

workdir/street-housenumbers-sashegy.txt: street-housenumbers.txt
	@mkdir -p workdir
	sed 's/3602714372/3602714524/' street-housenumbers.txt > workdir/street-housenumbers-sashegy.txt

workdir/street-housenumbers-nemetvolgy.txt: street-housenumbers.txt
	@mkdir -p workdir
	sed 's/3602714372/3602806940/' street-housenumbers.txt > workdir/street-housenumbers-nemetvolgy.txt

workdir/street-housenumbers-martonhegy.txt: street-housenumbers.txt
	@mkdir -p workdir
	sed 's/3602714372/3602787965/' street-housenumbers.txt > workdir/street-housenumbers-martonhegy.txt

workdir/street-housenumbers-ormezo.txt: street-housenumbers.txt
	@mkdir -p workdir
	sed 's/3602714372/3602713749/' street-housenumbers.txt > workdir/street-housenumbers-ormezo.txt

workdir/street-housenumbers-farkasvolgy.txt: street-housenumbers.txt
	@mkdir -p workdir
	sed 's/3602714372/3602788020/' street-housenumbers.txt > workdir/street-housenumbers-farkasvolgy.txt

workdir/street-housenumbers-magasut.txt: street-housenumbers.txt
	@mkdir -p workdir
	sed 's/3602714372/3602805999/' street-housenumbers.txt > workdir/street-housenumbers-magasut.txt

workdir/street-housenumbers-krisztinavaros.txt: street-housenumbers.txt
	@mkdir -p workdir
	sed 's/3602714372/3602786310/' street-housenumbers.txt > workdir/street-housenumbers-krisztinavaros.txt

workdir/street-housenumbers-lagymanyos.txt: street-housenumbers.txt
	@mkdir -p workdir
	sed 's/3602714372/3602702615/' street-housenumbers.txt > workdir/street-housenumbers-lagymanyos.txt
