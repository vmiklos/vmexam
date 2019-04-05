check:
	pycodestyle embed
	! pylint embed 2>&1 | egrep -i 'unused|indent'
	mypy --strict embed
