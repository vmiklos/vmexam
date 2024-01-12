-include config.mak

ifneq ($(DEBUG),)
WEBPACK_OPTIONS = --mode=development --devtool inline-source-map
else
WEBPACK_OPTIONS = --mode=production
endif

INSTDIR=$(HOME)/git/vmiklos.hu/htdocs/osm/missing-housenumbers/

all: bundle.js ol.css

bundle.js: main.ts tsconfig.json webpack.config.js package-lock.json Makefile config.mak
	npx webpack ${WEBPACK_OPTIONS} --config webpack.config.js

ol.css: node_modules/ol/ol.css package-lock.json
	cp node_modules/ol/ol.css .

package-lock.json: package.json
	npm install
	touch $@

run:
	python3 -m http.server

check:
	npx eslint main.ts

install:
	cp *.kml index.html ol.css bundle.js $(INSTDIR)
