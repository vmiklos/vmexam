#include <stdio.h> // for printf()
#include <getopt.h>
#include <stdlib.h> // for exit()
#include <limits.h> // for PATH_MAX
#include <string.h> // for strcpy()
#include "foo.h"

char fwo_conffile[PATH_MAX];
int fwo_verbose = 0;
int fwo_version = 0;
int fwo_ignore = 0;

int usage(const char *myname)
{
	printf("usage: %s [options]\n", myname);
	printf("-v | --verbose <level>   Verbose mode.\n");
	printf("-V | --version           Version info.\n");
	printf("-c | --config  <file>    Config file.\n");
	printf("     --ignore            Ignore something.\n");
	exit(0);
}

int main(int argc, char *argv[])
{
	strcpy(fwo_conffile, CONFFILE);
	int opt;
	int option_index;
	static struct option opts[] =
	{
		{"version",        no_argument,       0, 'V'},
		{"verbose",        required_argument, 0, 'v'},
		{"config",         required_argument, 0, 'c'},
		{"ignore",         no_argument,       0, 1000},
		{0, 0, 0, 0}
	};
	
	if (argc < 2)
	{
		usage(argv[0]);
	}
	
	while((opt = getopt_long(argc, argv, "c:v:V", opts, &option_index)))
	{
		if(opt < 0)
		{
			break;
		}
		switch(opt)
		{
			case 1000: fwo_ignore = 1; break;
			case 'c': strcpy(fwo_conffile, optarg); break;
			case 'v': fwo_verbose = atoi(optarg); break;
			case 'V': fwo_version = 1; break;
		}
	}
	printf("Config file: %s\n", fwo_conffile);
	if(fwo_ignore)
	{
		printf("Ignoring something\n");
	}
	if(fwo_version)
	{
		printf("%s\n", FOOVER);
		return(0);
	}
	if(fwo_verbose)
	{
		printf("Verbose level: %d\n", fwo_verbose);
	}
	printf("Input files:\n");
	while(optind < argc)
	{
		printf("%s\n", argv[optind]);
		optind++;
	}
	return(0);
}
