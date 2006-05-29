#include <alpm.h>
#include <stdio.h>

void cb_log(unsigned short level, char *msg)
{
	printf("%s\n", msg);
}

int main()
{
	PM_DB *db_local, *db_fwcurr;
	PM_LIST *pmpkgs, *lp, *junk;

	if(alpm_initialize("/home/vmiklos/darcs/examples/libalpm/deps/t") == -1)
		fprintf(stderr, "failed to initilize alpm library (%s)\n", alpm_strerror(pm_errno));
	if(alpm_set_option(PM_OPT_LOGCB, (long)cb_log) == -1)
		printf("failed to set option LOGCB (%s)\n", alpm_strerror(pm_errno));
	if((db_local = alpm_db_register("local"))==NULL)
		fprintf(stderr, "could not register 'local' database (%s)\n", alpm_strerror(pm_errno));
	if((db_fwcurr = alpm_db_register("frugalware-current"))==NULL)
		fprintf(stderr, "could not register 'frugalware-current' database (%s)\n", alpm_strerror(pm_errno));
	if(alpm_trans_init(PM_TRANS_TYPE_SYNC, PM_TRANS_FLAG_NOCONFLICTS, NULL, NULL, NULL) == -1)
		fprintf(stderr, "failed to init transaction (%s)\n", alpm_strerror(pm_errno));
	if(alpm_trans_addtarget("ncurses"))
		fprintf(stderr, "failed to add target 'ncurses' (%s)\n", alpm_strerror(pm_errno));
	if(alpm_trans_addtarget("zlib"))
		fprintf(stderr, "failed to add target 'zlib' (%s)\n", alpm_strerror(pm_errno));
	if(alpm_trans_addtarget("glibc"))
		fprintf(stderr, "failed to add target 'glibc' (%s)\n", alpm_strerror(pm_errno));

	if(alpm_trans_prepare(&junk) == -1)
		fprintf(stderr, "failed to prepare transaction (%s)\n", alpm_strerror(pm_errno));

	pmpkgs = alpm_trans_getinfo(PM_TRANS_PACKAGES);
	for(lp = alpm_list_first(pmpkgs); lp; lp = alpm_list_next(lp))
	{
		PM_SYNCPKG *sync = alpm_list_getdata(lp);
		PM_PKG *pkg = alpm_sync_getinfo(sync, PM_SYNC_PKG);
		printf("%s-%s-%s%s\n",
			(char*)alpm_pkg_getinfo(pkg, PM_PKG_NAME),
			(char*)alpm_pkg_getinfo(pkg, PM_PKG_VERSION),
			(char*)alpm_pkg_getinfo(pkg, PM_PKG_ARCH),
			PM_EXT_PKG);
	}
	alpm_trans_release();
	return(0);
}
