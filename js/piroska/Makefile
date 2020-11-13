-include config.mak

ifneq ($(DEBUG),)
BROWSERIFY_OPTIONS = --debug
else
BROWSERIFY_OPTIONS = --plugin tinyify
endif

bundle.js: main.js package-lock.json Makefile config.mak
	node_modules/.bin/browserify -o bundle.js $(BROWSERIFY_OPTIONS) main.js

check:
	eslint main.js

run:
	python3 -m http.server
