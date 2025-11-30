run:
	dx serve

install:
	rm -rf target/dx/rubik-scramble/release/web/public/
	dx build --release
	cd target/dx/rubik-scramble/release/web/public/ && rsync -avP --delete-after . ~/git/vmiklos.hu/pages/rubik-scramble/
