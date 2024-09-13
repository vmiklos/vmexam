build: Makefile requirements.txt
	mypy --python-version 3.11 --strict --no-error-summary exec_client.py
	mypy --python-version 3.11 --strict --no-error-summary exec_server.py

check: build check-format check-lint

check-format:
	flake8 exec_client.py
	flake8 exec_server.py

check-lint:
	pylint exec_client.py
	pylint exec_server.py
