#include <stdio.h>
#include <unistd.h>
#include <limits.h>

int main()
{
	char cwd[PATH_MAX];
	getcwd(cwd, PATH_MAX);
	printf("%s\n", cwd);
}
