run:
	dx serve

install:
	dx build --release
	cd target/dx/skel/release/web/public/ && rsync -avP . wilson:share/pages/dioxus/skel/
