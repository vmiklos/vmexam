-include config.mak

ifneq ($(DEBUG),)
WEBPACK_OPTIONS = --mode=development --devtool inline-source-map
else
WEBPACK_OPTIONS = --mode=production
endif

bundle.js: main.ts rubik.ts tsconfig.json webpack.config.js package-lock.json Makefile config.mak
	npx webpack ${WEBPACK_OPTIONS} --config webpack.config.js

package-lock.json: package.json
	npm install

config.mak:
	touch config.mak

check:
	npx eslint main.ts rubik.ts

run:
	python3 -m http.server

install:
	cp index.html bundle.js ~/git/vmiklos.hu/pages/rubik/
