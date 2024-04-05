-include config.mak

ifneq ($(DEBUG),)
WEBPACK_OPTIONS = --mode=development --devtool inline-source-map
else
WEBPACK_OPTIONS = --mode=production
endif

bundle.js: main.ts tsconfig.json webpack.config.js package-lock.json Makefile config.mak
	npx webpack ${WEBPACK_OPTIONS} --config webpack.config.js

package-lock.json: package.json
	npm install
	touch $@

check: check-unit check-lint

# Without coverage: tools/run-cypress.sh
check-unit: bundle.js
	rm -rf .nyc_output/ coverage/
	npx webpack --mode=development --devtool inline-source-map --config webpack-cov.config.js
	tools/run-cypress.sh --env COVERAGE_RUN=1
	npx nyc report --statements 100 --check-coverage --reporter=text-summary
	npx nyc report --reporter=html
	rm bundle.js

check-unit-interactive:
	npx cypress open

check-lint:
	npx eslint main.ts cypress/e2e/main.cy.ts

run:
	python3 -m http.server

install:
	scp index.html bundle.js wilson:share/pages/addr-osmify/
