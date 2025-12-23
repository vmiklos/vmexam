run:
	dx serve

install:
	rm -rf target/dx/loim/release/web/public/
	dx build --release
	cd target/dx/loim/release/web/public/ && rsync -avP --delete-after . ~/git/vmiklos.hu/pages/loim/
