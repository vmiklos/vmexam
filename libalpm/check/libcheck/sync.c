/*
 *  sync.c
 * 
 *  Copyright (c) 2002-2006 by Judd Vinet <jvinet@zeroflux.org>
 * 
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program; if not, write to the Free Software
 *  Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307, 
 *  USA.
 */

#if defined(__APPLE__) || defined(__OpenBSD__)
#include <sys/syslimits.h>
#include <sys/stat.h>
#endif

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>
#include <dirent.h>
#ifdef CYGWIN
#include <limits.h> /* PATH_MAX */
#endif

#include <alpm.h>
/* pacman */
#include "util.h"
#include "log.h"
#include "download.h"
#include "list.h"
#include "package.h"
#include "db.h"
#include "trans.h"
#include "sync.h"
#include "conf.h"

extern config_t *config;

extern list_t *pmc_syncs;

static int sync_synctree(list_t *syncs)
{
	char *root, *dbpath;
	char path[PATH_MAX];
	list_t *i;
	int success = 0, ret, oldrepos = 0;

	alpm_get_option(PM_OPT_ROOT, (long *)&root);
	alpm_get_option(PM_OPT_DBPATH, (long *)&dbpath);

	for(i = syncs; i; i = i->next) {
		list_t *files = NULL;
		char newmtime[16] = "";
		char lastupdate[16] = "";
		sync_t *sync = (sync_t *)i->data;

		/* get the lastupdate time */
		db_getlastupdate(sync->db, lastupdate);
		if(strlen(lastupdate) == 0) {
			vprint("failed to get lastupdate time for %s (no big deal)\n", sync->treename);
		}

		/* build a one-element list */
		snprintf(path, PATH_MAX, "%s" PM_EXT_DB, sync->treename);
		files = list_add(files, strdup(path));

		snprintf(path, PATH_MAX, "%s%s", root, dbpath);

		ret = downloadfiles_forreal(sync->servers, path, files, lastupdate, newmtime);
		FREELIST(files);
		if(ret > 0) {
			ERR(NL, "failed to synchronize %s\n", sync->treename);
			success--;
		} else if(ret >= 0) {
			oldrepos++;
		}
	}

	return(success < 0 ? success : oldrepos);
}

int pacman_sync(int mode)
{
	int confirm = 0;
	int retval = 0;
	list_t *i;
	PM_LIST *packages, *data, *lp;
	char *root, *cachedir;
	char ldir[PATH_MAX];
	int varcache = 1;
	list_t *files = NULL;

	if(pmc_syncs == NULL || !list_count(pmc_syncs)) {
		ERR(NL, "no usable package repositories configured.\n");
		return(-1);
	}

	/* open the database(s) */
	for(i = pmc_syncs; i; i = i->next) {
		sync_t *sync = i->data;
		sync->db = alpm_db_register(sync->treename);
		if(sync->db == NULL) {
			ERR(NL, "%s\n", alpm_strerror(pm_errno));
			return(-1);
		}
	}

	if(!mode) {
		/* grab a fresh package list */
		alpm_logaction("synchronizing package lists");
		return(sync_synctree(pmc_syncs));
	}

	/* Step 1: create a new transaction...
	 */
	if(alpm_trans_init(PM_TRANS_TYPE_SYNC, config->flags, cb_trans_evt, cb_trans_conv, cb_trans_progress) == -1) {
		return(-1);
	}

		alpm_logaction("starting full system upgrade");
		if(alpm_trans_sysupgrade() == -1) {
			ERR(NL, "%s\n", alpm_strerror(pm_errno));
			alpm_trans_release();
			return(-1);
		}

		data = alpm_trans_getinfo(PM_TRANS_PACKAGES);
		retval = (alpm_list_count(data) > 0);
		alpm_trans_release();
		return(retval);
}

/* vim: set ts=2 sw=2 noet: */
