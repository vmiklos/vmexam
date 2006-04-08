#include <curl/curl.h>
#include <sys/stat.h>
#include <stdio.h>
#include <limits.h>
#include <string.h>
#include "util.h"

int main(int argc, char **argv)
{
	CURL *easyhandle;
	FILE *fp;
	struct stat props;
	char path[PATH_MAX];

	if(argc != 3)
	{
		printf("usage: %s url file\n", argv[0]);
		return(1);
	}
	
	if (stat(argv[2], &props) != 0)
	{
		perror(argv[2]);
		return(1);
	}
	
	if (S_ISDIR(props.st_mode))
		snprintf(path, PATH_MAX, "%s/%s", argv[2], "baz");
	else
		strcpy(path, argv[2]);

	if ((fp = fopen(path, "w")) == NULL)
	{
		perror("could not open file for writing");
		return(1);
	}
	
	if (curl_global_init(CURL_GLOBAL_ALL) != 0)
		cerror(__FILE__, __LINE__);
	if ((easyhandle = curl_easy_init()) == NULL)
		cerror(__FILE__, __LINE__);

	/*if (curl_easy_setopt(easyhandle, CURLOPT_VERBOSE, 1) != 0)
		cerror(__FILE__, __LINE__);*/
	if (curl_easy_setopt(easyhandle, CURLOPT_WRITEDATA, fp) != 0)
		cerror(__FILE__, __LINE__);
	if (curl_easy_setopt(easyhandle, CURLOPT_URL, argv[1]) != 0)
		cerror(__FILE__, __LINE__);
	if (curl_easy_setopt(easyhandle, CURLOPT_NOPROGRESS, 0) != 0)
		cerror(__FILE__, __LINE__);
	if (curl_easy_perform(easyhandle) != 0)
		cerror(__FILE__, __LINE__);
	else
		printf("done downloading %s to %s\n", argv[1], path);
	
	curl_easy_cleanup(easyhandle);
	curl_global_cleanup();
}
