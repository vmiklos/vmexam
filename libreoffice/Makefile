NAME := $(shell grep $(USER) /etc/passwd|cut -d : -f 5|cut -d' ' -f 1)
REGRESSIONS_BLAME_NAME_URL = https://bugs.documentfoundation.org/buglist.cgi?f1=longdesc&f2=keywords&o1=substring&o2=anywords&query_format=advanced&resolution=---&v1=Adding%20Cc%3A%20to%20$(NAME)&v2=regression

PYTHON_FILES = \
	       find-unsorted-includes \
	       includetools.py \
	       qa/empty-atom.py

check: download
	pycodestyle $(PYTHON_FILES)
	TDOC="workdir/regressions-blame-$(NAME).atom" qa/empty-atom.py

download:
	@mkdir -p workdir
	curl -o workdir/regressions-blame-$(NAME).atom "$(REGRESSIONS_BLAME_NAME_URL)&ctype=atom"
