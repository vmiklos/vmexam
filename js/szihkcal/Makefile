-include config.mak

ifneq ($(DEBUG),)
# the source map here survives both tsc and the bundling
BROWSERIFY_OPTIONS = --debug
else
# tinyify works fine on the tsc output
BROWSERIFY_OPTIONS = --plugin tinyify
endif

bundle.js: main.ts package-lock.json Makefile config.mak
	node_modules/.bin/browserify -o bundle.js $(BROWSERIFY_OPTIONS) --plugin tsify main.ts

package-lock.json: package.json
	npm install
	touch $@

check: check-lint

check-lint:
	node_modules/.bin/eslint main.ts

run:
	python3 -m http.server
