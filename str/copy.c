#include <stdio.h>

#ifdef HAVE_STRING_H
#include <string.h>
#else
int strcpy(char *to, const char *from)
{
	for(;(*to=*from)!=0;from++, to++);
}

int strlen(const char *input)
{
	char *copy;
	for (copy = input; *copy != 0; copy++);
	return(copy - input);
}
#endif

int main(int argc, char *argv[])
{
	char output[strlen(argv[1])];

	strcpy(output, argv[1]);
	printf("%s\n", output);
}
