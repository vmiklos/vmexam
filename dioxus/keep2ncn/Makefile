run:
	dx serve

install:
	rm -rf target/dx/keep2ncn/release/web/public/
	dx build --release
	cd target/dx/keep2ncn/release/web/public/ && rsync -avP --delete-after . ~/git/vmiklos.hu/pages/keep2ncn/
