run:
	dx serve

install:
	rm -rf target/dx/skel/release/web/public/
	dx build --release
	cd target/dx/skel/release/web/public/ && rsync -avP --delete-after . ~/git/vmiklos.hu/pages/skel/
