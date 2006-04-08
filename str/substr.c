#include <string.h>
#include <stdio.h>

int substr(char *to, char *from, int start, int length)
{
	unsigned int i=0;
	unsigned int enabled=0;
	length += start;
	for(;*from!='\0'; from++, i++)
	{
		if(i==start)
			enabled=1;
		if(i==length)
			enabled=0;
		if(enabled==1)
		{
			*to=*from;
			to++;
		}
	}
	*to='\0';
	return(0);
}

int main ()
{
	char a[]="abcdefgh";
	char b[8];

	substr(b, a, 4, 2); // ef from abcdefgh
	
	printf("%s\n", b);
}
