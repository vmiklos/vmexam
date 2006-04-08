#include <rsync/rsync.h>

int main(int argc,char *argv[])
{
	if (argc !=3)
	{
		printf("usage: %s from to\n", argv[0]);
		return(1);
	}
	return(rsync(argc, argv));
}
