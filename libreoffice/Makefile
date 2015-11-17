REGRESSIONS_BLAME_URL = https://bugs.documentfoundation.org/buglist.cgi?f1=longdesc&f2=keywords&o1=substring&o2=anywords&query_format=advanced&resolution=---&v1=Adding%20Cc%3A%20to%20$(USER)&v2=regression&ctype=atom

PYTHON_FILES = \
	       find-unneeded-includes \
	       find-unsorted-includes \
	       includetools.py \
	       qa/empty-atom.py

check: download
	pep8 --ignore=E501 $(PYTHON_FILES)
	#TDOC="workdir/regressions-blame-$(USER).atom" qa/empty-atom.py

download:
	@mkdir -p workdir
	curl -o workdir/regressions-blame-$(USER).atom "$(REGRESSIONS_BLAME_URL)"
