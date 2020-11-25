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

check: check-unit check-lint

check-unit: bundle.js
	tools/run-cypress.sh

check-unit-interactive:
	node_modules/.bin/cypress open

check-lint:
	node_modules/.bin/eslint main.ts cypress/integration/main.spec.ts

run:
	python3 -m http.server

check-unit-cov:
	rm -rf .nyc_output/ coverage/
	node_modules/.bin/browserify -o bundle.js --transform browserify-istanbul --plugin [ tsify --target=ES2017 ] main.ts
	tools/run-cypress.sh --env COVERAGE_RUN=1
	node_modules/.bin/nyc report --statements 100 --check-coverage --reporter=text-summary
	node_modules/.bin/nyc report --reporter=html
	rm bundle.js
