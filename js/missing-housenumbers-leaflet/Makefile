-include config.mak

ifneq ($(DEBUG),)
BROWSERIFY_OPTIONS = --debug
else
BROWSERIFY_OPTIONS = --plugin tinyify
endif

INSTDIR=$(HOME)/git/vmiklos.hu/htdocs/osm/missing-housenumbers/

all: bundle.js leaflet.css

bundle.js: main.js package-lock.json Makefile config.mak
	node_modules/.bin/browserify -o bundle.js $(BROWSERIFY_OPTIONS) main.js

leaflet.css: package-lock.json
	cp node_modules/leaflet/dist/leaflet.css .

package-lock.json: package.json
	npm install
	touch package-lock.json

check: check-lint

check-lint:
	node_modules/eslint/bin/eslint.js main.js

run:
	python3 -m http.server

install:
	cp *.json index.html leaflet.css bundle.js $(INSTDIR)
