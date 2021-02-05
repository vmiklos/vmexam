build: Makefile
	mypy --python-version 3.6 --strict --no-error-summary mbr_dumper.py

check: build check-format check-lint

check-format:
	flake8 mbr_dumper.py

check-lint:
	pylint mbr_dumper.py
