-include config.mak

bundle.js: main.js Makefile
ifneq ($(DEBUG),)
	browserify --debug -o bundle.js main.js
else
	browserify -g uglifyify -o bundle.js main.js
endif

check:
	eslint main.js

run:
	python3 simple-https-server.py
