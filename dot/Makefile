all: frugalware.png

frugalware.png: frugalware.dot
	dot frugalware.dot -Tpng -o frugalware.png

frugalware.dot: frugalware.fdb
	python fdb2dot.py frugalware >frugalware.dot

frugalware.fdb:
	rsync -avP rsync://rsync.frugalware.org/pub/frugalware/frugalware-stable/frugalware-i686/frugalware.fdb .

clean:
	rm -f frugalware.*
