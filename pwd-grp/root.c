#include <stdio.h>
#include <sys/types.h>

int main()
{
	uid_t myuid;

	// see if we're root or not
	myuid = geteuid();
	if(!myuid && getenv("FAKEROOTKEY"))
	{
		// fakeroot doesn't count, we're non-root
		myuid = 99;
	}
	if(myuid)
	{
		printf("Permission denied!\n");
	}
}
