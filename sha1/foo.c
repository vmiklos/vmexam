#include <stdio.h>
#include "sha1.h"

int main(int argc, char *argv[])
{
	char *ret;
	
	if (argc < 2)
	{
		printf("usage: %s file\n", argv[0]);
		return(1);
	}
	if ((ret = SHAFile(argv[1])) != NULL)
	{
		printf("sha1sum(%s) == %s\n", argv[1], ret);
	}
	else
		return(1);
}
