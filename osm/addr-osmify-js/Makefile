-include config.mak

ifneq ($(DEBUG),)
BROWSERIFY_OPTIONS = --debug
else
BROWSERIFY_OPTIONS = --plugin tinyify
endif

bundle.js: main.js package-lock.json Makefile config.mak
	browserify -o bundle.js $(BROWSERIFY_OPTIONS) main.js

package-lock.json: package.json
	npm install

check:
	node_modules/eslint/bin/eslint.js main.js

run:
	python3 -m http.server
