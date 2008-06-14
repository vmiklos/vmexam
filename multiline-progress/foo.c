#include <stdio.h>

int main()
{
	char *esc = "\x1b[A\r\x1b[K";
	int i, j;

	putchar('\n');

	for (i = 0; i < 10; i++) {
		for (j = 0; j < 10; j++) {
			printf("%smain: %d/9\nsub:  %d/9\r", esc, i, j);
			fflush(stdout);
			sleep(1);
		}
	}
}
