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

# Without coverage: tools/run-cypress.sh
check-unit:
	rm -rf .nyc_output/ coverage/
	npx webpack --config webpack-cov.config.js
	tools/run-cypress.sh --env COVERAGE_RUN=1
	npx nyc report --statements 100 --check-coverage --reporter=text-summary
	npx nyc report --reporter=html

check-lint:
	npx eslint main.js cypress/e2e/main.cy.js

run:
	python3 -m http.server
