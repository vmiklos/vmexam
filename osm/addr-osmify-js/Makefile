-include config.mak

ifneq ($(DEBUG),)
WEBPACK_OPTIONS = --mode=development
else
WEBPACK_OPTIONS = --mode=production
endif

bundle.js: main.js webpack.config.js package-lock.json Makefile config.mak
	npx webpack ${WEBPACK_OPTIONS} --config webpack.config.js

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
	npx webpack --config webpack-cov.config.js
	tools/run-cypress.sh --env COVERAGE_RUN=1
	node_modules/.bin/nyc report --statements 100 --check-coverage --reporter=text-summary
	node_modules/.bin/nyc report --reporter=html
