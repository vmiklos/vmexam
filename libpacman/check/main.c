#include <stdio.h>
#include <check.h>

main()
{
	int x;

	if (paccheck(0))
		printf("Sy needed\n");
	else
		printf("Sy not needed\n");

	if (paccheck(1))
		printf("Su needed\n");
	else
		printf("Su not needed\n");

	paccleanup(0);
}
