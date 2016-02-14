ifeq ($(OS),Android)
ANDROID_OPTS = -DCMAKE_TOOLCHAIN_FILE=$(PWD)/android/android.toolchain.cmake -DANDROID_NDK=$(HOME)/git/libreoffice/master-android/external/android-ndk
endif

all:
	@mkdir -p workdir
ifneq ($(OS),Windows_NT)
	cd workdir && cmake -DCMAKE_INSTALL_PREFIX:PATH=$(PWD)/instdir -DCMAKE_BUILD_TYPE=Debug $(ANDROID_OPTS) ..
else
	cd workdir && cmake -DCMAKE_INSTALL_PREFIX:PATH=$(shell cygpath -m $(PWD))/instdir -DCMAKE_BUILD_TYPE=Debug ..
endif
	cd workdir && cmake --build . --target install

clean:
	rm -rf instdir
	rm -rf workdir

run:
ifeq ($(OS),Android)
	adb push instdir/bin/hello /data/local/tmp/hello
	adb shell /data/local/tmp/hello
else
	instdir/bin/hello
endif
