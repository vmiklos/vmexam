/*
 *  log.h
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
#ifndef _PM_LOG_H
#define _PM_LOG_H

#define MSG(line, fmt, args...) pm_fprintf(stdout, line, fmt, ##args)
#define ERR(line, fmt, args...) do { \
	pm_fprintf(stderr, line, "error: "); \
	pm_fprintf(stderr, CL, fmt, ##args); \
} while(0)
#define WARN(line, fmt, args...) do { \
	pm_fprintf(stderr, line, "warning: "); \
	pm_fprintf(stderr, CL, fmt, ##args); \
} while(0)

enum {
	NL, /* new line */
	CL  /* current line */
};

void pm_fprintf(FILE *file, unsigned short line, char *fmt, ...);
void vprint(char *fmt, ...);

int yesno(char *fmt, ...);

#endif /* _PM_LOG_H */

/* vim: set ts=2 sw=2 noet: */
