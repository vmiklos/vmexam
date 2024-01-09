-include config.mak

ifneq ($(DEBUG),)
WEBPACK_OPTIONS = --mode=development
else
WEBPACK_OPTIONS = --mode=production
endif

bundle.js: main.ts tsconfig.json webpack.config.js package-lock.json Makefile config.mak
	npx webpack ${WEBPACK_OPTIONS} --config webpack.config.js

package-lock.json: package.json
	npm install
	touch package-lock.json

check:
	npx eslint main.ts

install:
	scp index.html bundle.js smiley.svg wilson:share/ovis-ora/

run:
	python3 -m http.server
