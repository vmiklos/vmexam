#include <stdio.h>
#include <stdlib.h>
#include <signal.h>

static void sighandler(int x)
{
	switch(x)
	{
		case SIGINT:
			printf("^C\n");
			break;
		case SIGTERM:
			printf("kill\n");
			break;
		case SIGHUP:
			printf("kill -HUP\n");
			break;
		case SIGSEGV:
			printf("segmantation fault\n");
			exit(1);
			break;
		case SIGFPE:
			printf("floating point exception (divison by zero or so)\n");
			exit(1);
			break;
	}
}

int main()
{
	int *x=NULL;

	// comments: not reproducable with the lines below
	signal(SIGTERM, sighandler);
	signal(SIGHUP, sighandler);
	signal(SIGINT, sighandler);
	signal(SIGQUIT, sighandler); // Quit from keyboard
	signal(SIGBUS, sighandler);  // bus error
	signal(SIGSEGV, sighandler);
	signal(SIGILL, sighandler);  // illegal instruction
	signal(SIGFPE, sighandler);
	signal(SIGABRT, sighandler); // abort()

	//sleep(10); // ^C/kill/kill -HUP helper
	//*x=0/0; // divison by zero
	*x=0; // segfault
}
