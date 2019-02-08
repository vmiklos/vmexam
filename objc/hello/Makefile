OBJC_FLAGS = $(shell gnustep-config --objc-flags)
main: main.m Makefile
	clang $(OBJC_FLAGS) -o main -x objective-c main.m -fconstant-string-class=NSConstantString -fobjc-nonfragile-abi -fblocks -lobjc -lgnustep-base -ldispatch -I/usr/include/GNUstep -L/usr/GNUstep/Local/Library/Libraries/
