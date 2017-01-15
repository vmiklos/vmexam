-include config.mak

bundle.js: main.js Makefile
ifneq ($(DEBUG),)
	browserify --debug -o bundle.js main.js
else
	browserify -o bundle.max.js main.js
	minify -o bundle.js bundle.max.js
endif

check:
	eslint main.js
