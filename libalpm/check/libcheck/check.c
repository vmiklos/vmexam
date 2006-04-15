/*
 *  check.c
 * 
 *  Copyright (c) 2006 by Miklos Vajna <vmiklos@frugalware.org>
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

#include <stdlib.h>
#include <stdio.h>
#include <limits.h>
#include <getopt.h>
#include <string.h>
#include <signal.h>
#include <unistd.h>
#if defined(__APPLE__)
#include <malloc/malloc.h>
#elif defined(__OpenBSD__) || defined(__APPLE__)
#include <sys/malloc.h>
#include <sys/types.h>
#elif defined(CYGWIN)
#include <libgen.h> /* basename */
#else
#include <mcheck.h> /* debug */
#endif

#include <alpm.h>
/* pacman */
#include "list.h"
#include "util.h"
#include "log.h"
#include "download.h"
#include "conf.h"
#include "package.h"
#include "sync.h"

#define PACCONF "/etc/pacman.conf"

#if defined(__OpenBSD__) || defined(__APPLE__)
#define BSD
#endif

config_t *config = NULL;

PM_DB *db_local;
/* list of (sync_t *) structs for sync locations */
list_t *pmc_syncs = NULL;

int maxcols = 80;

extern int neednl;

void paccleanup(int signum)
{
	list_t *lp;

	if(signum != 0 && config->op_d_vertest == 0) {
		fprintf(stderr, "\n");
	}

	/* free alpm library resources */
	alpm_release();

	/* free memory */
	for(lp = pmc_syncs; lp; lp = lp->next) {
		sync_t *sync = lp->data;
		list_t *i;
		for(i = sync->servers; i; i = i->next) {
			server_t *server = i->data;
			FREE(server->protocol);
			FREE(server->server);
			FREE(server->path);
		}
		FREELIST(sync->servers);
		FREE(sync->treename);
	}
	FREELIST(pmc_syncs);
	FREECONF(config);

	exit(signum);
}

// 0: Sy, 1: Su
int paccheck(int mode)
{
	int ret = 0;
	char *cenv = NULL;
	list_t *lp;

	/* set signal handlers */
	signal(SIGINT, paccleanup);
	signal(SIGTERM, paccleanup);

	/* init config data */
	config = config_new();
	config->debug |= PM_LOG_WARNING;

	if(config->root == NULL) {
		config->root = strdup(PM_ROOT);
	}

	/* add a trailing '/' if there isn't one */
	if(config->root[strlen(config->root)-1] != '/') {
		char *ptr;
		MALLOC(ptr, strlen(config->root)+2);
		strcpy(ptr, config->root);
		strcat(ptr, "/");
		FREE(config->root);
		config->root = ptr;
	}

	/* initialize pm library */
	if(alpm_initialize(config->root) == -1) {
		ERR(NL, "failed to initilize alpm library (%s)\n", alpm_strerror(pm_errno));
		paccleanup(1);
	}

	if(config->configfile == NULL) {
		config->configfile = strdup(PACCONF);
	}
	if(parseconfig(config->configfile, config) == -1) {
		paccleanup(1);
	}

	/* set library parameters */
	if(alpm_set_option(PM_OPT_LOGMASK, (long)config->debug) == -1) {
		ERR(NL, "failed to set option LOGMASK (%s)\n", alpm_strerror(pm_errno));
		paccleanup(1);
	}
	if(alpm_set_option(PM_OPT_DBPATH, (long)config->dbpath) == -1) {
		ERR(NL, "failed to set option DBPATH (%s)\n", alpm_strerror(pm_errno));
		paccleanup(1);
	}
	if(alpm_set_option(PM_OPT_CACHEDIR, (long)config->cachedir) == -1) {
		ERR(NL, "failed to set option CACHEDIR (%s)\n", alpm_strerror(pm_errno));
		paccleanup(1);
	}

	for(lp = config->op_s_ignore; lp; lp = lp->next) {
		if(alpm_set_option(PM_OPT_IGNOREPKG, (long)lp->data) == -1) {
			ERR(NL, "failed to set option IGNOREPKG (%s)\n", alpm_strerror(pm_errno));
			paccleanup(1);
		}
	}
	/* query dbpath */
	alpm_get_option(PM_OPT_DBPATH, (long *)config->dbpath);
	
	/* Opening local database */
	db_local = alpm_db_register("local");
	if(db_local == NULL) {
		ERR(NL, "could not register 'local' database (%s)\n", alpm_strerror(pm_errno));
		paccleanup(1);
	}

	ret = pacman_sync(mode);
	alpm_release();
	return(ret);
}

/* vim: set ts=2 sw=2 noet: */
