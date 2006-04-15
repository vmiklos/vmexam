/*
 *  log.c
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

#include <stdio.h>
#include <string.h>
#include <stdarg.h>
#include <ctype.h>
#include <time.h>

#include <alpm.h>
/* pacman */
#include "log.h"
#include "list.h"
#include "conf.h"

#define LOG_STR_LEN 256

extern config_t *config;

int neednl; /* for cleaner message output */

/* Wrapper to fprintf() that allows to choose if we want the output
 * to be appended on the current line, or written to a new one
 */
void pm_fprintf(FILE *file, unsigned short line, char *fmt, ...)
{
	va_list args;

	char str[LOG_STR_LEN];

	if(neednl == 1 && line == NL) {
		fprintf(stdout, "\n");
		neednl = 0;
	}

	va_start(args, fmt);
	vsnprintf(str, LOG_STR_LEN, fmt, args);
	va_end(args);

	fprintf(file, str);
	fflush(file);

	neednl = (str[strlen(str)-1] == 10) ? 0 : 1;
}

/* Check verbosity option and, if set, print the
 * string to stdout
 */
void vprint(char *fmt, ...)
{
	va_list args;

	char str[LOG_STR_LEN];

	if(config->verbose > 0) {
		va_start(args, fmt);
		vsnprintf(str, LOG_STR_LEN, fmt, args);
		va_end(args);
		pm_fprintf(stdout, NL, str);
	}
}

/* presents a prompt and gets a Y/N answer
 */
int yesno(char *fmt, ...)
{
	char str[LOG_STR_LEN];
	char response[32];
	va_list args;

	if(config->noconfirm) {
		return(1);
	}

	va_start(args, fmt);
	vsnprintf(str, LOG_STR_LEN, fmt, args);
	va_end(args);
	MSG(NL, str);

	if(fgets(response, 32, stdin)) {
		/* trim whitespace and newlines */
		char *pch = response;
		while(isspace(*pch)) {
			pch++;
		}
		if(pch != response) {
			memmove(response, pch, strlen(pch) + 1);
		}
		pch = response + strlen(response) - 1;
		while(isspace(*pch)) {
			pch--;
		}
		*++pch = 0;
		strtrim(response);

		if(!strcasecmp(response, "Y") || !strcasecmp(response, "YES") || !strlen(response)) {
			return(1);
		}
	}
	return(0);
}

/* vim: set ts=2 sw=2 noet: */
