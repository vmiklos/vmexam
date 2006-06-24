#include <stdio.h>
#include <stdlib.h>
#include <limits.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <string.h>

int gpg(char *keys, char *file)
{
	int ret=0, fd=0, i=0;
	char cmd[PATH_MAX+1];
	char *args[512];
	pid_t pid;
	struct stat buf;

	if(stat("/usr/bin/gpgv", &buf))
		return(1);

	if((fd = open("/dev/null", O_WRONLY))==-1)
		return(1);
	pid = fork();
	if (pid < 0)
		return(1);
	else if(pid==0)
	{
		args[i++] = strdup("gpgv");
		args[i++] = strdup("--logger-fd");
		asprintf(&args[i++], "%d", fd);
		args[i++] = strdup("--keyring");
		args[i++] = keys;
		args[i++] = file;
		ret = execv("/usr/bin/gpgv", args);
		for(i=0;i<4;i++)
			free(args[i]);
		exit(ret);
	}
	else
		while(wait(&ret) != pid);
	close(fd);
	return(!ret);
}

int main()
{
	if(gpg("/home/vmiklos/.gnupg/pubring.gpg", "good/pacman-3.3.2.tar.gz.asc"))
		printf("ok\n");
	else
		printf("failed\n");
	if(gpg("/home/vmiklos/.gnupg/pubring.gpg", "bad/pacman-3.3.2.tar.gz.asc"))
		printf("ok\n");
	else
		printf("failed\n");
	return(0);
}
