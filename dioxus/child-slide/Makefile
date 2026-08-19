run:
	dx serve

install:
	rm -rf target/dx/child-slide/release/web/public/
	dx build --release
	cd target/dx/child-slide/release/web/public/ && rsync -avP --delete-after --exclude='*.json' . ~/git/vmiklos.hu/pages/child-slide/
