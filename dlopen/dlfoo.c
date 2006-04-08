#include <stdlib.h>
#include <stdio.h>
#include <dlfcn.h>

int main(int argc, char **argv)
{
	void *handle;
	double (*cos)(double);
	char *error;

	if(argc<2)
	{
		printf("usage: %s number\n", argv[0]);
		return(1);
	}

	handle = dlopen ("/lib/libm.so.6", RTLD_LAZY);
	if (!handle)
	{
		fprintf(stderr, "%s\n", dlerror());
		exit(1);
	}

	cos = dlsym(handle, "cos");
	if ((error = dlerror()) != NULL)
	{
		fprintf(stderr, "%s\n", dlerror());
		exit(1);
	}

	printf ("%f\n", (*cos)(atoi(argv[1])));
	dlclose(handle);
	return(0);
}
