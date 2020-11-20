-include config.mak

# openlayers requires these options
BROWSERIFY_OPTIONS = -g [ babelify --presets [ "@babel/preset-env" ] ]

ifneq ($(DEBUG),)
# the source map here survives both tsc and the bundling
BROWSERIFY_OPTIONS += --debug
else
# tinyify works fine on the tsc output
BROWSERIFY_OPTIONS += --plugin tinyify
endif

INSTDIR=$(HOME)/git/vmiklos.hu/htdocs/osm/missing-housenumbers/

all: bundle.js ol.css

bundle.js: main.ts package-lock.json Makefile config.mak
	browserify -o bundle.js $(BROWSERIFY_OPTIONS) --plugin tsify main.ts

ol.css: node_modules/ol/ol.css package-lock.json
	cp node_modules/ol/ol.css .

package-lock.json: package.json
	npm install
	touch $@

run:
	python3 -m http.server

check:
	node_modules/eslint/bin/eslint.js main.ts

install:
	cp *.kml index.html ol.css bundle.js $(INSTDIR)
