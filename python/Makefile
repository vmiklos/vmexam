PYFILES := sleepavg rejourn2pelican

check:
	pycodestyle $(PYFILES)
	! pylint $(PYFILES) 2>&1 | egrep -i 'unused|indent'
	for i in $(PYFILES); do \
		mypy --strict $$i; \
	done

sleepavg.bin: sleepavg.cxx
	g++ -std=c++11 -o sleepavg.bin sleepavg.cxx
