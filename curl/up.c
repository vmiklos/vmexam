#include <curl/curl.h>
#include <stdio.h>
#include <limits.h>
#include "util.h"

int main(int argc, char **argv)
{
	CURL *easyhandle;
	FILE *fp;
	char path[PATH_MAX];

	if(argc != 3)
	{
		printf("usage: %s file url \n", argv[0]);
		return(1);
	}

	snprintf(path, PATH_MAX, "%s/%s", argv[2], argv[1]);

	if ((fp = fopen(argv[1], "r")) == NULL)
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
	if (curl_easy_setopt(easyhandle, CURLOPT_READDATA, fp) != 0)
		cerror(__FILE__, __LINE__);
	if (curl_easy_setopt(easyhandle, CURLOPT_URL, path) != 0)
		cerror(__FILE__, __LINE__);
	if (curl_easy_setopt(easyhandle, CURLOPT_UPLOAD, 1) != 0)
		cerror(__FILE__, __LINE__);
	if (curl_easy_perform(easyhandle) != 0)
		cerror(__FILE__, __LINE__);
	else
		printf("done uploading %s to %s\n", argv[1], argv[2]);
	
	curl_easy_cleanup(easyhandle);
	curl_global_cleanup();
}
