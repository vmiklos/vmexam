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
	touch $@

check: check-lint

check-lint:
	npx eslint main.ts rubik.ts

run:
	python3 -m http.server

install:
	scp index.html bundle.js wilson:share/pages/rubik/
