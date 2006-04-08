#include <stdio.h>
#include "md5.h"

int main(int argc, char *argv[])
{
	char *ret;
	
	if (argc < 2)
	{
		printf("usage: %s file\n", argv[0]);
		return(1);
	}
	if ((ret = MDFile(argv[1])) != NULL)
	{
		printf("md5sum(%s) == %s\n", argv[1], ret);
	}
	else
		return(1);
}
