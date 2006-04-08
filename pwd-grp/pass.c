#include <stdio.h>
#include <limits.h>
#include <unistd.h>

int main()
{
	char *pass;
	const char prompt[]="Password:";
	pass = getpass(prompt);
	// we now have the password in pass
}
