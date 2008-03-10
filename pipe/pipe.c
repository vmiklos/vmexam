/*
 *  pipe.c
 * 
 *  Copyright (c) 2007 by Miklos Vajna <vmiklos@frugalware.org>
 *  Some ideas by William C. Benton, see
 *  http://www.cs.wisc.edu/~willb/537/pipe.c
 * 
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program; if not, write to the Free Software
 *  Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307, 
 *  USA.
 */

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/types.h>
#include <wait.h>

int popen2(char **args, FILE **fpin, FILE **fpout)
{
	int pin[2], pout[2];
	pid_t pid;

	if(pipe(pin) == -1)
	{
		perror("pipe");
		return(-1);
	}
	if(pipe(pout) == -1)
	{
		perror("pipe");
		return(-1);
	}

	pid = fork();
	if(pid == -1)
	{
		perror("fork");
		return(-1);
	}
	if (pid == 0) {
		/* we are "in" the child */

		/* this process' stdin should be the read end of the pin pipe.
		 * dup2 closes original stdin */
		dup2(pin[0], STDIN_FILENO);
		close(pin[0]);
		close(pin[1]);
		dup2(pout[1], STDOUT_FILENO);
		/* this process' stdout should be the write end of the pout
		 * pipe. dup2 closes original stdout */
		close(pout[1]);
		close(pout[0]);

		execvp(args[0], args);
		/* on sucess, execv never returns */
		return(-1);
	}

	close(pin[0]);
	close(pout[1]);
	*fpin = fdopen(pin[1], "w");
	*fpout = fdopen(pout[0], "r");
	return(0);
}

int main(int argc, char **argv)
{
	FILE *pin, *pout;
	char buf[256];
	char *args[] = { "bc", NULL };

	if(popen2(args, &pin, &pout) == -1)
		return(1);

	fprintf(pin, "2+2\n");
	fclose(pin);

	while(!feof(pout))
	{
		fgets(buf, 255, pout);
		printf("%s", buf);
		buf[0] = '\0';
	}
	fclose(pout);

	return(0);
}

