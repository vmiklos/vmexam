LO_HOME = /opt/libreoffice

run:
	PYTHONPATH=$(LO_HOME)/program URE_BOOTSTRAP=file://$(LO_HOME)/program/fundamentalrc ./test.py "file://$(LO_HOME)/sdk/examples/cpp/DocumentLoader/test.odt"

check:
	pycodestyle test.py

