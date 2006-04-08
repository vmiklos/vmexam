#include <stdio.h>
#include <string.h>

int main(int argc, char **argv)
{
	if (argc != 2 || strlen(argv[1])!=1)
	{
		printf("usage: %s char\n", argv[0]);
		return(1);
	}
	printf("%d\n", *argv[1]);
}
