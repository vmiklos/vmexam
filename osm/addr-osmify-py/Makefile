check: check-flake8 check-mypy check-pylint

check-flake8:
	flake8 addr_osmify.py
	flake8 test_addr_osmify.py

check-mypy:
	mypy --python-version 3.5 --strict addr_osmify.py
	mypy --python-version 3.5 --strict test_addr_osmify.py

check-pylint:
	pylint addr_osmify.py
	pylint test_addr_osmify.py

check-unit:
	coverage run --branch --module unittest test_addr_osmify.py
	coverage report --show-missing --fail-under=100 addr_osmify.py
