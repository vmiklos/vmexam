#include <curl/curl.h>
#include <stdio.h>
#include "util.h"

int main(int argc, char **argv)
{
	CURL *easyhandle;
	FILE *fp;

	if(argc != 3)
	{
		printf("usage: %s url file\n", argv[0]);
		return(1);
	}

	if ((fp = fopen(argv[2], "w")) == NULL)
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
	if (curl_easy_perform(easyhandle) != 0)
		cerror(__FILE__, __LINE__);
	else
		printf("done downloading %s to %s\n", argv[1], argv[2]);
	
	curl_easy_cleanup(easyhandle);
	curl_global_cleanup();
}
