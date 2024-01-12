-include config.mak

ifneq ($(DEBUG),)
WEBPACK_OPTIONS = --mode=development --devtool inline-source-map
else
WEBPACK_OPTIONS = --mode=production
endif

all: bundle.js leaflet.css

bundle.js: main.ts tsconfig.json webpack.config.js package-lock.json Makefile config.mak
	npx webpack ${WEBPACK_OPTIONS} --config webpack.config.js

leaflet.css: package-lock.json
	cp node_modules/leaflet/dist/leaflet.css .

package-lock.json: package.json
	npm install
	touch package-lock.json

check: check-lint

check-lint:
	npx eslint main.ts

run:
	python3 -m http.server

install:
ifneq ($(INSTDIR),)
	cp pois.json pois.xml index.html leaflet.css bundle.js $(INSTDIR)
endif
