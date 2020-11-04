-include config.mak

ifneq ($(DEBUG),)
BROWSERIFY_OPTIONS = --debug
else
BROWSERIFY_OPTIONS = --plugin tinyify
endif

bundle.js: main.js package-lock.json Makefile config.mak
	node_modules/.bin/browserify -o bundle.js $(BROWSERIFY_OPTIONS) main.js

package-lock.json: package.json
	npm install
	touch package-lock.json

check: check-unit check-lint

check-unit:
	tools/run-cypress.sh

check-lint:
	node_modules/eslint/bin/eslint.js main.js cypress/integration/main.spec.js

run:
	python3 -m http.server

check-unit-cov:
	rm -rf .nyc_output/ coverage/
	node_modules/.bin/browserify -o bundle.js -t browserify-istanbul main.js
	tools/run-cypress.sh --env COVERAGE_RUN=1
	node_modules/.bin/nyc report --statements 100 --check-coverage --reporter=text-summary
	node_modules/.bin/nyc report --reporter=html
