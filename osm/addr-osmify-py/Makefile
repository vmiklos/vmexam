build: Makefile
	mypy --python-version 3.11 --strict --no-error-summary addr_osmify.py
	mypy --python-version 3.11 --strict --no-error-summary test_addr_osmify.py

check: build check-unit check-format check-lint
	@echo "make check: ok"

check-format:
	flake8 addr_osmify.py
	flake8 test_addr_osmify.py

check-lint:
	pylint addr_osmify.py
	pylint test_addr_osmify.py

check-unit:
	coverage run --branch --module unittest test_addr_osmify.py
	coverage report --show-missing --fail-under=100 addr_osmify.py

tags:
	ctags --python-kinds=-iv --fields=+l --extra=+q -R --totals=yes *
