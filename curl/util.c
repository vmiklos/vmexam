#include <stdio.h>
#include <stdlib.h>

void cerror(char *file, int line)
{
	printf("sg went wrong at %s (line %d)\n", file, line);
	exit(1);
}


