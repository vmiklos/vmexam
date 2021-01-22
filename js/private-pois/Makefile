-include config.mak

ifneq ($(DEBUG),)
BROWSERIFY_OPTIONS = --debug
else
BROWSERIFY_OPTIONS = --plugin tinyify
endif

all: bundle.js leaflet.css

bundle.js: main.ts package-lock.json Makefile config.mak
	node_modules/.bin/browserify -o bundle.js $(BROWSERIFY_OPTIONS) --plugin tsify main.ts

leaflet.css: package-lock.json
	cp node_modules/leaflet/dist/leaflet.css .

package-lock.json: package.json
	npm install
	touch package-lock.json

check: check-lint

check-lint:
	node_modules/eslint/bin/eslint.js main.ts

run:
	python3 -m http.server

install:
ifneq ($(INSTDIR),)
	cp pois.json index.html leaflet.css bundle.js $(INSTDIR)
endif
