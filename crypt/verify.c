#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <crypt.h>


int
main(int argc, char **argv)
{
	if(argc < 2)
	{
		printf("usage: %s hash [pass]\n", argv[0]);
		return(1);
	}

	char *result;
	int ok;


	if (argc < 3)
		/* Read in the user's password and encrypt it,
			 passing the expected password in as the salt. */
		result = crypt(getpass("Password:"), argv[1]);
	else
		result = crypt(argv[2], argv[1]);


	/* Test the result. */
	ok = strcmp (result, argv[1]) == 0;


	puts(ok ? "Access granted." : "Access denied.");
	return ok ? 0 : 1;
}
