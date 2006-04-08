#include <sys/stat.h>
#include <pwd.h>
#include <grp.h>
#include <stdio.h>

int main(int argc, char *argv[])
{
	struct stat props;
	struct passwd *o;
	struct group *g;
	
	if(argc<=1)
	{
		printf("usage: %s file\n", argv[0]);
		return(1);
	}
	
	if (stat(argv[1], &props) != 0)
	{
		perror(argv[1]);
		return(1);
	}
	
	o = getpwuid(props.st_uid);
	g = getgrgid(props.st_gid);
	
	printf("%s.%s\n", o->pw_name, g->gr_name);
}
