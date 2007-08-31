all: frugalware-current.png

frugalware-current.png: frugalware-current.dot
	dot frugalware-current.dot -Tpng -o frugalware-current.png

frugalware-current.dot: frugalware-current.fdb
	python fdb2dot.py frugalware-current >frugalware-current.dot

frugalware-current.fdb:
	rsync -avP rsync://rsync.frugalware.org/pub/frugalware/frugalware-current/frugalware-i686/frugalware-current.fdb .

clean:
	rm -f frugalware-current.*
