run:
	dx serve

run-android:
	ANDROID_HOME=~/Android/Sdk/ ANDROID_NDK_HOME=~/Android/Sdk/ndk/current/ dx serve --platform android

install:
	rm -rf target/dx/rubik-scramble/release/web/public/
	dx build --release
	cd target/dx/rubik-scramble/release/web/public/ && rsync -avP --delete-after . ~/git/vmiklos.hu/pages/rubik-scramble/
