#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/types.h>
#include <wait.h>

// some ides from William C. Benton, http://www.cs.wisc.edu/~willb/537/pipe.c

int main(int argc, char *argv[])
{
	/* first, create the pipe, fds[0] is the read end of pipe, fds[1]
	 * is the write end of pipe. 
	 */
	int pin[2], pout[2];
	char buf[256];
	char *ptr;
	int i;
	FILE *fp;
	pipe(pin);
	pipe(pout);

	/* fork two children. remember that a return value of 0 means that
	 * we are "in" the child. the parent is ignoring the return value
	 * of both forks. 
	 */
	if (fork() == 0) {
		dup2(pin[0], STDIN_FILENO);   /* this process' stdin should
						 be the read end of the pin
						 pipe. dup2 closes original
						 stdin */
		close(pin[0]);
		close(pin[1]);
		dup2(pout[1], STDOUT_FILENO); /* this process' stdout should
						 be the write end of the pout
						 pipe. dup2 closes original
						 stdout */
		close(pout[1]);                /* don't need this after dup2 */
		close(pout[0]);

		char *const args[] = { "/usr/bin/bc", NULL };
		execv(args[0], args);
		_exit(EXIT_FAILURE);  /* on sucess, execv never returns */
	}

	/* the parent process only needed the pipe to give to its
	 * children, it should NOT keep these open (can you figure out
	 * why? what happens if the parent does not close them?). 
	 */
	close(pin[0]);
	fp = fdopen(pin[1], "w");
	fprintf(fp, "2+2\n");
	fclose(fp);
	close(pout[1]);
	fp = fdopen(pout[0], "r");
	fgets(buf, 255, fp);
	fclose(fp);

	/* wait for the two children we forked */
	wait(NULL);

	printf("res: '%s'\n", buf);

	/* do you think it matters what order the children were forked? */

	return 0;
}
