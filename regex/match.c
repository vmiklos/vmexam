#include <stdio.h>
#include <regex.h>

int reg_match(char *string, char *pattern)
{
	int result;
	regex_t reg;
	
	regcomp(&reg, pattern, REG_EXTENDED | REG_NOSUB);
	result = regexec(&reg, string, 0, 0, 0);
	regfree(&reg);
	return(!(result));
}

int main(int argc, char ** argv)
{
	if (argc != 3)
	{
		printf("usage: %s string pattern\n", argv[0]);
		return(1);
	}
	if(reg_match(argv[1], argv[2]))
		printf("'%s' matches.\n", argv[1]);
	else
		printf("No match!\n");
	return(0);
}
