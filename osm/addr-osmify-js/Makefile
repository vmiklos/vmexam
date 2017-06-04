-include config.mak

bundle.js: main.js Makefile
ifneq ($(DEBUG),)
	browserify --debug -o bundle.js main.js
else
	browserify -t uglifyify -o bundle.js main.js
endif

check:
	eslint main.js
