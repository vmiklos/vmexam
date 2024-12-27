PYTHON_OBJECTS = pdfcal.py

all:

check: check-mypy check-flake8 check-pylint
	@echo "make check: ok"

check-mypy: $(PYTHON_OBJECTS) Makefile requirements.txt
	mypy --python-version 3.11 --strict --no-error-summary $(PYTHON_OBJECTS) && touch $@

check-flake8: $(patsubst %.py,%.flake8,$(PYTHON_OBJECTS))

check-pylint: $(patsubst %.py,%.pylint,$(PYTHON_OBJECTS))

%.flake8: %.py Makefile requirements.txt
	flake8 $< && touch $@

%.pylint : %.py Makefile .pylintrc requirements.txt
	pylint $< && touch $@
