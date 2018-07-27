RTF_REGRESSIONS_URL = https://bugs.documentfoundation.org/buglist.cgi?keywords=regression&keywords_type=allwords&status_whiteboard_type=allwordssubstr&query_format=advanced&status_whiteboard=filter:rtf&resolution=---
UNTAGGED_RTF_REGRESSIONS_URL = https://bugs.documentfoundation.org/buglist.cgi?keywords=regression&keywords_type=allwords&short_desc=RTF&resolution=---&query_format=advanced&short_desc_type=allwordssubstr&product=LibreOffice
REGRESSIONS_BLAME_URL = https://bugs.documentfoundation.org/buglist.cgi?f1=longdesc&f2=keywords&o1=substring&o2=anywords&query_format=advanced&resolution=---&v1=Adding%20Cc%3A%20to%20$(USER)&v2=regression
NAME := $(shell grep $(USER) /etc/passwd|cut -d : -f 5|cut -d' ' -f 1)
REGRESSIONS_BLAME_NAME_URL = https://bugs.documentfoundation.org/buglist.cgi?f1=longdesc&f2=keywords&o1=substring&o2=anywords&query_format=advanced&resolution=---&v1=Adding%20Cc%3A%20to%20$(NAME)&v2=regression

PYTHON_FILES = \
	       find-unsorted-includes \
	       includetools.py \
	       qa/empty-atom.py

check: download
	pycodestyle $(PYTHON_FILES)
	TDOC="workdir/rtf-regressions.atom" qa/empty-atom.py
	TDOC="workdir/untagged-rtf-regressions.atom" qa/empty-atom.py
	TDOC="workdir/regressions-blame-$(USER).atom" qa/empty-atom.py
	TDOC="workdir/regressions-blame-$(NAME).atom" qa/empty-atom.py

download:
	@mkdir -p workdir
	curl -o workdir/rtf-regressions.atom "$(RTF_REGRESSIONS_URL)&ctype=atom"
	curl -o workdir/untagged-rtf-regressions.atom "$(UNTAGGED_RTF_REGRESSIONS_URL)&ctype=atom"
	curl -o workdir/regressions-blame-$(USER).atom "$(REGRESSIONS_BLAME_URL)&ctype=atom"
	curl -o workdir/regressions-blame-$(NAME).atom "$(REGRESSIONS_BLAME_NAME_URL)&ctype=atom"
