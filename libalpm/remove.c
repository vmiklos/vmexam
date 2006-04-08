#include <stdio.h>
#include <stdlib.h>
#include <alpm.h>

// callback to handle transaction events
void event(unsigned char event, void *data1, void *data2)
{
	switch(event)
	{
		case PM_TRANS_EVT_REMOVE_START:
			printf("removing %s... ",
				(char *)alpm_pkg_getinfo(data1, PM_PKG_NAME));
			fflush(stdout);
			break;
		case PM_TRANS_EVT_REMOVE_DONE:
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
	PM_DB *db_local;
	PM_LIST *data;

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

	db_local = alpm_db_register("local");
	if(db_local == NULL)
	{
		fprintf(stderr, "could not register 'local' database (%s)\n",
			alpm_strerror(pm_errno));
		return(1);
	}

	if(alpm_trans_init(PM_TRANS_TYPE_REMOVE, 0, event, NULL, NULL) == -1)
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
					printf("\t%s: is required by %s\n", (char*)alpm_dep_getinfo(miss, PM_DEP_TARGET), (char*)alpm_dep_getinfo(miss, PM_DEP_NAME));
				}
				alpm_list_free(data);
				break;
			default:
				break;
		}
		cleanup(1);
	}
	if(alpm_trans_commit(NULL) == -1)
	{
		fprintf(stderr, "failed to commit transaction (%s)\n",
			alpm_strerror(pm_errno));
		cleanup(1);
	}
	cleanup(0);
	return(0);
}
