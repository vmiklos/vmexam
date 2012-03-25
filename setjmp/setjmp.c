#include <setjmp.h>
#include <stdio.h>

int main()
{
	printf("init\n");
	jmp_buf env;
	if (!setjmp(env))
	{
		printf("first\n");
		longjmp(env, 1);
	} else {
		printf("second\n");
	}
	printf("end\n");
}
