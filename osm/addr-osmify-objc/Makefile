main: main.m Makefile
	clang -fobjc-arc -g -Wall -Wextra -Werror -framework Foundation -o main main.m

check:
	clang -fobjc-arc --analyze main.m
