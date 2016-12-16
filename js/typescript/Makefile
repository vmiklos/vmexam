# tsc --help|grep 'errors\? \(on\|when\)'|grep -v 'Do not'
TSCFLAGS := \
	   --noImplicitAny \
	   --noImplicitReturns \
	   --noImplicitThis \
	   --noUnusedLocals \
	   --noUnusedParameters \

greet.js: greet.ts Makefile
	tsc $(TSCFLAGS) greet.ts
