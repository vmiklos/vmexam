/*
 *  download.c
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

#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>
#include <time.h>
#include <sys/time.h>
#include <ftplib.h>

#include <alpm.h>
/* pacman */
#include "util.h"
#include "log.h"
#include "list.h"
#include "download.h"
#include "conf.h"

/* progress bar */
static char sync_fnm[25];
static int offset;
static struct timeval t0, t;
static float rate;
static int xfered1;
static unsigned char eta_h, eta_m, eta_s;

/* pacman options */
extern config_t *config;

extern int maxcols;

static int log_progress(netbuf *ctl, int xfered, void *arg)
{
	int fsz = *(int*)arg;
	int pct = ((float)(xfered+offset) / fsz) * 100;
	int i, cur;
	struct timeval t1;
	float timediff;
	/* a little hard to conceal easter eggs in open-source software, but
	 * they're still fun.  ;)
	 */
	static unsigned short mouth;
	static unsigned int   lastcur = 0;

	if(config->noprogressbar) {
		return(1);
	}

	gettimeofday(&t1, NULL);
	if(xfered+offset == fsz) {
		t = t0;
	}
	timediff = t1.tv_sec-t.tv_sec + (float)(t1.tv_usec-t.tv_usec) / 1000000;

	if(xfered+offset == fsz) {
		/* average download rate */
		rate = xfered / (timediff * 1024);
		/* total download time */
		eta_s = (int)timediff;
		eta_h = eta_s / 3600;
		eta_s -= eta_h * 3600;
		eta_m = eta_s / 60;
		eta_s -= eta_m * 60;
	} else if(timediff > 1) {
		/* we avoid computing the rate & ETA on too small periods of time, so that
		   results are more significant */
		rate = (xfered-xfered1) / (timediff * 1024);
		xfered1 = xfered;
		gettimeofday(&t, NULL);
		eta_s = (fsz-(xfered+offset)) / (rate * 1024);
		eta_h = eta_s / 3600;
		eta_s -= eta_h * 3600;
		eta_m = eta_s / 60;
		eta_s -= eta_m * 60;
	}

	printf(" %s [", sync_fnm);
	cur = (int)((maxcols-64)*pct/100);
	for(i = 0; i < maxcols-64; i++) {
		if(config->chomp) {
			if(i < cur) {
				printf("-");
			} else {
				if(i == cur) {
					if(lastcur == cur) {
						if(mouth) {
							printf("\033[1;33mC\033[m");
						} else {
							printf("\033[1;33mc\033[m");
						}
					} else {
						mouth = mouth == 1 ? 0 : 1;
						if(mouth) {
							printf("\033[1;33mC\033[m");
						} else {
							printf("\033[1;33mc\033[m");
						}
					}
				} else {
					printf("\033[0;37m*\033[m");
				}
			}
		} else {
			(i < cur) ? printf("#") : printf(" ");
		}
	}
	if(rate > 1000) {
		printf("] %3d%%  %6dK  %6.0fK/s  %02d:%02d:%02d\r", pct, ((xfered+offset) / 1024), rate, eta_h, eta_m, eta_s);
	} else {
		printf("] %3d%%  %6dK  %6.1fK/s  %02d:%02d:%02d\r", pct, ((xfered+offset) / 1024), rate, eta_h, eta_m, eta_s);
	}
	lastcur = cur;
	fflush(stdout);
	return(1);
}

static int copyfile(char *src, char *dest)
{
	FILE *in, *out;
	size_t len;
	char buf[4097];

	in = fopen(src, "r");
	if(in == NULL) {
		return(1);
	}
	out = fopen(dest, "w");
	if(out == NULL) {
		return(1);
	}

	while((len = fread(buf, 1, 4096, in))) {
		fwrite(buf, 1, len, out);
	}

	fclose(in);
	fclose(out);
	return(0);
}

/*
 * Download a list of files from a list of servers
 *   - if one server fails, we try the next one in the list
 *
 * RETURN:  0 for successful download, 1 on error
 */
int downloadfiles(list_t *servers, const char *localpath, list_t *files)
{
	return(!!downloadfiles_forreal(servers, localpath, files, NULL, NULL));
}

/*
 * This is the real downloadfiles, used directly by sync_synctree() to check
 * modtimes on remote files.
 *   - if *mtime1 is non-NULL, then only download files
 *     if they are different than *mtime1.  String should be in the form
 *     "YYYYMMDDHHMMSS" to match the form of ftplib's FtpModDate() function.
 *   - if *mtime2 is non-NULL, then it will be filled with the mtime
 *     of the remote file (from MDTM FTP cmd or Last-Modified HTTP header).
 * 
 * RETURN:  0 for successful download
 *         -1 if the mtimes are identical
 *          1 on error
 */
int downloadfiles_forreal(list_t *servers, const char *localpath,
		list_t *files, const char *mtime1, char *mtime2)
{
	int fsz;
	netbuf *control = NULL;
	list_t *lp;
	int done = 0;
	list_t *complete = NULL;
	list_t *i;

	if(files == NULL) {
		return(0);
	}

	for(i = servers; i && !done; i = i->next) {
		server_t *server = (server_t*)i->data;

		if(!config->xfercommand && strcmp(server->protocol, "file")) {
			if(!strcmp(server->protocol, "ftp") && !config->proxyhost) {
				FtpInit();
				vprint("connecting to %s:21\n", server->server);
				if(!FtpConnect(server->server, &control)) {
					ERR(NL, "cannot connect to %s\n", server->server);
					continue;
				}
				if(!FtpLogin("anonymous", "arch@guest", control)) {
					ERR(NL, "anonymous login failed\n");
					FtpQuit(control);
					continue;
				}	
				if(!FtpChdir(server->path, control)) {
					ERR(NL, "could not cwd to %s: %s\n", server->path, FtpLastResponse(control));
					FtpQuit(control);
					continue;
				}
				if(!config->nopassiveftp) {
					if(!FtpOptions(FTPLIB_CONNMODE, FTPLIB_PASSIVE, control)) {
						WARN(NL, "failed to set passive mode\n");
					}
				} else {
					vprint("FTP passive mode not set\n");
				}
			} else if(config->proxyhost) {
				char *host;
				unsigned port;
				host = (config->proxyhost) ? config->proxyhost : server->server;
				port = (config->proxyport) ? config->proxyport : 80;
				if(strchr(host, ':')) {
					vprint("connecting to %s\n", host);
				} else {
					vprint("connecting to %s:%u\n", host, port);
				}
				if(!HttpConnect(host, port, &control)) {
					ERR(NL, "cannot connect to %s\n", host);
					continue;
				}
			}

			/* set up our progress bar's callback (and idle timeout) */
			if(strcmp(server->protocol, "file") && control) {
				FtpOptions(FTPLIB_IDLETIME, (long)1000, control);
				FtpOptions(FTPLIB_CALLBACKARG, (long)&fsz, control);
				FtpOptions(FTPLIB_CALLBACKBYTES, (10*1024), control);
			}
		}

		/* get each file in the list */
		for(lp = files; lp; lp = lp->next) {
			char *fn = (char *)lp->data;

			if(list_is_strin(fn, complete)) {
				continue;
			}

			if(config->xfercommand && strcmp(server->protocol, "file")) {
				int ret;
				int usepart = 0;
				char *ptr1, *ptr2;
				char origCmd[PATH_MAX];
				char parsedCmd[PATH_MAX] = "";
				char url[PATH_MAX];
				char cwd[PATH_MAX];
				/* build the full download url */
				snprintf(url, PATH_MAX, "%s://%s%s%s", server->protocol, server->server,
						server->path, fn);
				/* replace all occurrences of %o with fn.part */
				strncpy(origCmd, config->xfercommand, sizeof(origCmd));
				ptr1 = origCmd;
				while((ptr2 = strstr(ptr1, "%o"))) {
					usepart = 1;
					ptr2[0] = '\0';
					strcat(parsedCmd, ptr1);
					strcat(parsedCmd, fn);
					strcat(parsedCmd, ".part");
					ptr1 = ptr2 + 2;
				}
				strcat(parsedCmd, ptr1);
				/* replace all occurrences of %u with the download URL */
				strncpy(origCmd, parsedCmd, sizeof(origCmd));
				parsedCmd[0] = '\0';
				ptr1 = origCmd;
				while((ptr2 = strstr(ptr1, "%u"))) {
					ptr2[0] = '\0';
					strcat(parsedCmd, ptr1);
					strcat(parsedCmd, url);
					ptr1 = ptr2 + 2;
				}
				strcat(parsedCmd, ptr1);
				/* cwd to the download directory */
				getcwd(cwd, PATH_MAX);
				if(chdir(localpath)) {
					ERR(NL, "could not chdir to %s\n", localpath);
					return(1);
				}
				/* execute the parsed command via /bin/sh -c */
				vprint("running command: %s\n", parsedCmd);
				ret = system(parsedCmd);
				if(ret == -1) {
					ERR(NL, "running XferCommand: fork failed!\n");
					return(1);
				} else if(ret != 0) {
					/* download failed */
					vprint("XferCommand command returned non-zero status code (%d)\n", ret);
				} else {
					/* download was successful */
					complete = list_add(complete, fn);
					if(usepart) {
						char fnpart[PATH_MAX];
						/* rename "output.part" file to "output" file */
						snprintf(fnpart, PATH_MAX, "%s.part", fn);
						rename(fnpart, fn);
					}
				}
				chdir(cwd);
			} else {
				char output[PATH_MAX];
				int j, filedone = 0;
				char *ptr;
				struct stat st;
				snprintf(output, PATH_MAX, "%s/%s.part", localpath, fn);
				strncpy(sync_fnm, fn, 24);
				/* drop filename extension */
				ptr = strstr(fn, PM_EXT_DB);
				if(ptr && (ptr-fn) < 24) {
					sync_fnm[ptr-fn] = '\0';
				}
				ptr = strstr(fn, PM_EXT_PKG);
				if(ptr && (ptr-fn) < 24) {
					sync_fnm[ptr-fn] = '\0';
				}
				for(j = strlen(sync_fnm); j < 24; j++) {
					sync_fnm[j] = ' ';
				}
				sync_fnm[24] = '\0';
				offset = 0;

				/* ETA setup */
				gettimeofday(&t0, NULL);
				t = t0;
				rate = 0;
				xfered1 = 0;
				eta_h = 0;
				eta_m = 0;
				eta_s = 0;

				if(!strcmp(server->protocol, "ftp") && !config->proxyhost) {
					if(!FtpSize(fn, &fsz, FTPLIB_IMAGE, control)) {
						WARN(NL, "failed to get filesize for %s\n", fn);
					}
					/* check mtimes */
					if(mtime1) {
						char fmtime[64];
						if(!FtpModDate(fn, fmtime, sizeof(fmtime)-1, control)) {
							WARN(NL, "failed to get mtime for %s\n", fn);
						} else {
							strtrim(fmtime);
							if(mtime1 && !strcmp(mtime1, fmtime)) {
								/* mtimes are identical, skip this file */
								vprint("mtimes are identical, skipping %s\n", fn);
								filedone = -1;
								complete = list_add(complete, fn);
							} else {
								if(mtime2) {
									strncpy(mtime2, fmtime, 15); /* YYYYMMDDHHMMSS (=14b) */
									mtime2[14] = '\0';
								}
							}
						}
					}
					if(!filedone) {
						if(!stat(output, &st)) {
							offset = (int)st.st_size;
							if(!FtpRestart(offset, control)) {
								WARN(NL, "failed to resume download -- restarting\n");
								/* can't resume: */
								/* unlink the file in order to restart download from scratch */
								unlink(output);
							}
						}
						if(!FtpGet(output, fn, FTPLIB_IMAGE, control)) {
							ERR(NL, "\nfailed downloading %s from %s: %s\n", fn, server->server, FtpLastResponse(control));
							/* we leave the partially downloaded file in place so it can be resumed later */
						} else {
							filedone = 1;
						}
					}
				} else if(!strcmp(server->protocol, "http") || (config->proxyhost && strcmp(server->protocol, "file"))) {
					char src[PATH_MAX];
					char *host;
					unsigned port;
					struct tm fmtime1;
					struct tm fmtime2;
					memset(&fmtime1, 0, sizeof(struct tm));
					memset(&fmtime2, 0, sizeof(struct tm));
					if(!strcmp(server->protocol, "http") && !config->proxyhost) {
						/* HTTP servers hang up after each request (but not proxies), so
						 * we have to re-connect for each file.
						 */
						host = (config->proxyhost) ? config->proxyhost : server->server;
						port = (config->proxyhost) ? config->proxyport : 80;
						if(strchr(host, ':')) {
							vprint("connecting to %s\n", host);
						} else {
							vprint("connecting to %s:%u\n", host, port);
						}
						if(!HttpConnect(host, port, &control)) {
							ERR(NL, "cannot connect to %s\n", host);
							continue;
						}
						/* set up our progress bar's callback (and idle timeout) */
						if(strcmp(server->protocol, "file") && control) {
							FtpOptions(FTPLIB_IDLETIME, (long)1000, control);
							FtpOptions(FTPLIB_CALLBACKARG, (long)&fsz, control);
							FtpOptions(FTPLIB_CALLBACKBYTES, (10*1024), control);
						}
					}

					if(!stat(output, &st)) {
						offset = (int)st.st_size;
					}
					if(!config->proxyhost) {
						snprintf(src, PATH_MAX, "%s%s", server->path, fn);
					} else {
						snprintf(src, PATH_MAX, "%s://%s%s%s", server->protocol, server->server, server->path, fn);
					}
					if(mtime1 && strlen(mtime1)) {
						struct tm tmref;
						time_t t, tref;
						int diff;
						/* date conversion from YYYYMMDDHHMMSS to "rfc1123-date" */
						sscanf(mtime1, "%4d%2d%2d%2d%2d%2d",
						       &fmtime1.tm_year, &fmtime1.tm_mon, &fmtime1.tm_mday,
						       &fmtime1.tm_hour, &fmtime1.tm_min, &fmtime1.tm_sec);
						fmtime1.tm_year -= 1900;
						fmtime1.tm_mon--;
						/* compute the week day because some web servers (like lighttpd) need them. */
						/* we set tmref to "Thu, 01 Jan 1970 00:00:00" */
						memset(&tmref, 0, sizeof(struct tm));
						tmref.tm_mday = 1;
						tref = mktime(&tmref);
						/* then we compute the difference with mtime1 */
						t = mktime(&fmtime1);
						diff = ((t-tref)/3600/24)%7;
						fmtime1.tm_wday = diff+(diff >= 3 ? -3 : 4);

					}
					fmtime2.tm_year = 0;
					if(!HttpGet(server->server, output, src, &fsz, control, offset,
					            (mtime1) ? &fmtime1 : NULL, (mtime2) ? &fmtime2 : NULL)) {
						if(strstr(FtpLastResponse(control), "304")) {
							vprint("mtimes are identical, skipping %s\n", fn);
							filedone = -1;
							complete = list_add(complete, fn);
						} else {
							ERR(NL, "\nfailed downloading %s from %s: %s\n", src, server->server, FtpLastResponse(control));
							/* we leave the partially downloaded file in place so it can be resumed later */
						}
					} else {
						if(mtime2) {
							if(fmtime2.tm_year) {
								/* date conversion from "rfc1123-date" to YYYYMMDDHHMMSS */
								sprintf(mtime2, "%4d%02d%02d%02d%02d%02d",
								        fmtime2.tm_year+1900, fmtime2.tm_mon+1, fmtime2.tm_mday,
								        fmtime2.tm_hour, fmtime2.tm_min, fmtime2.tm_sec);
							} else {
								WARN(NL, "failed to get mtime for %s\n", fn);
							}
						}
						filedone = 1;
					}
				} else if(!strcmp(server->protocol, "file")) {
					char src[PATH_MAX];
					snprintf(src, PATH_MAX, "%s%s", server->path, fn);
					vprint("copying %s to %s/%s\n", src, localpath, fn);
					/* local repository, just copy the file */
					if(copyfile(src, output)) {
						ERR(NL, "failed copying %s\n", src);
					} else {
						filedone = 1;
					}
				}

				if(filedone > 0) {
					char completefile[PATH_MAX];
					complete = list_add(complete, fn);
					/* rename "output.part" file to "output" file */
					snprintf(completefile, PATH_MAX, "%s/%s", localpath, fn);
					rename(output, completefile);
				} else if(filedone < 0) {
					return(-1);
				}
				fflush(stdout);
			}
		}
		if(!config->xfercommand) {
			if(!strcmp(server->protocol, "ftp") && !config->proxyhost) {
				FtpQuit(control);
			} else if(!strcmp(server->protocol, "http") || (config->proxyhost && strcmp(server->protocol, "file"))) {
				HttpQuit(control);
			}
		}

		if(list_count(complete) == list_count(files)) {
			done = 1;
		}
	}

	return(!done);
}

char *fetch_pkgurl(char *target)
{
	char spath[PATH_MAX];
	char url[PATH_MAX];
	char *host, *path, *fn;
	struct stat buf;

	strncpy(url, target, PATH_MAX);
	host = strstr(url, "://");
	*host = '\0';
	host += 3;
	path = strchr(host, '/');
	*path = '\0';
	path++;
	fn = strrchr(path, '/');
	if(fn) {
		*fn = '\0';
		if(path[0] == '/') {
			snprintf(spath, PATH_MAX, "%s/", path);
		} else {
			snprintf(spath, PATH_MAX, "/%s/", path);
		}
		fn++;
	} else {
		fn = path;
		strcpy(spath, "/");
	}

	/* do not download the file if it exists in the current dir
	 */
	if(stat(fn, &buf) == 0) {
		vprint(" %s is already in the current directory\n", fn);
	} else {
		server_t *server;
		list_t *servers = NULL;
		list_t *files;

		MALLOC(server, sizeof(server_t));
		server->protocol = url;
		server->server = host;
		server->path = spath;
		servers = list_add(servers, server);

		files = list_add(NULL, fn);
		if(downloadfiles(servers, ".", files)) {
			ERR(NL, "failed to download %s\n", target);
			return(NULL);
		}
		FREELISTPTR(files);

		FREELIST(servers);
	}

	/* return the target with the raw filename, no URL */
	#if defined(__OpenBSD__) || defined(__APPLE__)
	return(strdup(fn));
	#else
	return(strndup(fn, PATH_MAX));
	#endif
}

/* vim: set ts=2 sw=2 noet: */
