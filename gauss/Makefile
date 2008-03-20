all: foo

test: foo
	./foo > tmp
	diff -u eredmenyek.txt tmp
	rm tmp

clean:
	rm -f foo tmp
