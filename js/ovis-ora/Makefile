-include config.mak

ifneq ($(DEBUG),)
# the source map here survives both tsc and the bundling
BROWSERIFY_OPTIONS = --debug
else
# tinyify works fine on the tsc output
BROWSERIFY_OPTIONS = --plugin tinyify
endif

bundle.js: main.ts package-lock.json Makefile config.mak
	browserify -o bundle.js $(BROWSERIFY_OPTIONS) --plugin [ tsify --noImplicitAny ] main.ts

package-lock.json: package.json
	npm install
	touch package-lock.json

check:
	node_modules/eslint/bin/eslint.js main.ts

install:
	scp index.html bundle.js smiley.svg wilson:share/ovis-ora/
