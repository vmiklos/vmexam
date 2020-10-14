-include config.mak

package-lock.json: package.json
	npm install

bundle.js: main.js package-lock.json Makefile
ifneq ($(DEBUG),)
	browserify --debug -o bundle.js main.js
else
	browserify --plugin tinyify -o bundle.js main.js
endif

check:
	eslint main.js
