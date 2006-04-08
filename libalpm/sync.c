#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <alpm.h>
#include <curl/curl.h>

// callback to handle transaction events
void event(unsigned char event, void *data1, void *data2)
{
	switch(event)
	{
		case PM_TRANS_EVT_ADD_START:
			printf("installing %s... ",
				(char *)alpm_pkg_getinfo(data1, PM_PKG_NAME));
			fflush(stdout);
			break;
		case PM_TRANS_EVT_UPGRADE_START:
			printf("upgrading %s... ",
				(char *)alpm_pkg_getinfo(data1, PM_PKG_NAME));
			fflush(stdout);
			break;
		case PM_TRANS_EVT_ADD_DONE:
		case PM_TRANS_EVT_UPGRADE_DONE:
			printf("done.\n");
			break;
	}
}

void cleanup(int ret)
{
	alpm_trans_release();
	alpm_release();
	exit(ret);
}

int main(int argc, char **argv)
{
	PM_DB *local, *frugalware;
	PM_LIST *data, *packages;
	char mirror[] = "http://www7.frugalware.org/pub/frugalware/frugalware-current/frugalware-i686";

	if(argc != 2)
	{
		printf("usage: %s pkgname\n", argv[0]);
		return(0);
	}

	if(alpm_initialize("/") == -1)
	{
		fprintf(stderr, "failed to initilize alpm library (%s)\n",
			alpm_strerror(pm_errno));
		return(1);
	}
	if(alpm_set_option(PM_OPT_DBPATH, (long)PM_DBPATH) == -1)
	{
		fprintf(stderr, "failed to set option DBPATH (%s)\n",
			alpm_strerror(pm_errno));
		cleanup(1);
	}

	local = alpm_db_register("local");
	if(local == NULL)
	{
		fprintf(stderr, "could not register 'local' database (%s)\n",
			alpm_strerror(pm_errno));
		return(1);
	}

	frugalware = alpm_db_register("frugalware-current");
	if(frugalware == NULL)
	{
		fprintf(stderr, "could not register 'frugalware-current' database (%s)\n",
			alpm_strerror(pm_errno));
		return(1);
	}

	// PM_TRANS_FLAG_ALLDEPS produces no interactive calls
	if(alpm_trans_init(PM_TRANS_TYPE_SYNC, PM_TRANS_FLAG_ALLDEPS, event, NULL, NULL) == -1)
	{
		fprintf(stderr, "failed to init transaction (%s)\n",
			alpm_strerror(pm_errno));
		cleanup(1);
	}
	if(alpm_trans_addtarget(argv[1]) == -1)
	{
		fprintf(stderr, "failed to add target '%s' (%s)\n",
			argv[1], alpm_strerror(pm_errno));
		cleanup(1);
	}
	if(alpm_trans_prepare(&data) == -1)
	{
		PM_LIST *lp;
		fprintf(stderr, "failed to prepare transaction (%s)\n",
			alpm_strerror(pm_errno));
		switch(pm_errno)
		{
			case PM_ERR_UNSATISFIED_DEPS:
				for(lp = alpm_list_first(data); lp; lp = alpm_list_next(lp))
				{
					PM_DEPMISS *miss = alpm_list_getdata(lp);
					printf("\t%s: is required by %s\n", (char*)alpm_dep_getinfo(miss, PM_DEP_TARGET),
						(char*)alpm_dep_getinfo(miss, PM_DEP_NAME));
				}
				alpm_list_free(data);
				break;
			case PM_ERR_CONFLICTING_DEPS:
				for(lp = alpm_list_first(data); lp; lp = alpm_list_next(lp))
				{
					PM_DEPMISS *miss = alpm_list_getdata(lp);

					printf("\t%s: conflicts with %s",
						(char*)alpm_dep_getinfo(miss, PM_DEP_TARGET),
						(char*)alpm_dep_getinfo(miss, PM_DEP_NAME));
				}
				alpm_list_free(data);
				break;
			default:
				break;
		}
		cleanup(1);
	}

	// download
	packages = alpm_trans_getinfo(PM_TRANS_PACKAGES);
	PM_LIST *lp;
	CURL *easyhandle;
	FILE *fp;
	curl_global_init(CURL_GLOBAL_ALL);

	for(lp = alpm_list_first(packages); lp; lp = alpm_list_next(lp))
	{
		PM_SYNCPKG *sync = alpm_list_getdata(lp);
		PM_PKG *spkg = alpm_sync_getinfo(sync, PM_SYNC_PKG);
		char url[PATH_MAX], pkgpath[PATH_MAX], targetpath[PATH_MAX];

		snprintf(pkgpath, PATH_MAX, "%s-%s-%s" PM_EXT_PKG, (char*)alpm_pkg_getinfo(spkg, PM_PKG_NAME),
			(char*)alpm_pkg_getinfo(spkg, PM_PKG_VERSION), (char*)alpm_pkg_getinfo(spkg, PM_PKG_ARCH));
		snprintf(url, PATH_MAX, "%s/%s", mirror, pkgpath);
		snprintf(targetpath, PATH_MAX, PM_ROOT PM_CACHEDIR "/%s", pkgpath);

		if ((fp = fopen(targetpath, "w")) == NULL)
		{
			perror("could not open file for writing");
			cleanup(1);
		}
		easyhandle = curl_easy_init();
		curl_easy_setopt(easyhandle, CURLOPT_WRITEDATA, fp);
		curl_easy_setopt(easyhandle, CURLOPT_URL, url);
		if (curl_easy_perform(easyhandle) != 0)
		{
			printf("failed to download %s\n", pkgpath);
			cleanup(1);
		}
		else
		{
			printf("downloaded %s\n", pkgpath);
			fflush(stdout);
		}
		curl_easy_cleanup(easyhandle);
		fclose(fp);
	}
	curl_global_cleanup();

	if(alpm_trans_commit(&data) == -1)
	{
		fprintf(stderr, "failed to commit transaction (%s)\n",
			alpm_strerror(pm_errno));
		cleanup(1);
	}
	cleanup(0);
	return(0);
}
