run:
	dx serve

install:
	rm -rf target/dx/random-choose/release/web/public/
	dx build --release
	cd target/dx/random-choose/release/web/public/ && rsync -avP --delete-after . ~/git/vmiklos.hu/pages/random-choose/
