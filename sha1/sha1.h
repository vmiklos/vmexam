/* Declarations of functions and data types used for SHA1 sum
   library functions.
   Copyright (C) 2000, 2001, 2003 Free Software Foundation, Inc.

   This program is free software; you can redistribute it and/or modify it
   under the terms of the GNU General Public License as published by the
   Free Software Foundation; either version 2, or (at your option) any
   later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program; if not, write to the Free Software Foundation,
   Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307, USA.  */

#include <stdio.h>
#include <limits.h>

#define rol(x,n) ( ((x) << (n)) | ((x) >> (32 -(n))) )
/* The code below is from md5.h (from coreutils), little modifications */
#define UINT_MAX_32_BITS 4294967295U

#if UINT_MAX == UINT_MAX_32_BITS
    typedef unsigned int sha_uint32;
#else
#if USHRT_MAX == UINT_MAX_32_BITS
    typedef unsigned short sha_uint32;
#else
#if ULONG_MAX == UINT_MAX_32_BITS
    typedef unsigned long sha_uint32;
#else
    /* The following line is intended to evoke an error. Using #error is not portable enough.  */
#error "Cannot determine unsigned 32-bit data type"
#endif
#endif
#endif
/* We have to make a guess about the integer type equivalent in size
   to pointers which should always be correct.  */
typedef unsigned long int sha_uintptr;

/* Structure to save state of computation between the single steps.  */
struct sha_ctx
{
  sha_uint32 A;
  sha_uint32 B;
  sha_uint32 C;
  sha_uint32 D;
  sha_uint32 E;

  sha_uint32 total[2];
  sha_uint32 buflen;
  char buffer[128];
};


/* Initialize structure containing state of computation. */
extern void sha_init_ctx (struct sha_ctx *ctx);

/* Starting with the result of former calls of this function (or the
   initialization function update the context for the next LEN bytes
   starting at BUFFER.
   It is necessary that LEN is a multiple of 64!!! */
extern void sha_process_block (const void *buffer, size_t len,
			       struct sha_ctx *ctx);

/* Starting with the result of former calls of this function (or the
   initialization function update the context for the next LEN bytes
   starting at BUFFER.
   It is NOT required that LEN is a multiple of 64.  */
extern void sha_process_bytes (const void *buffer, size_t len,
			       struct sha_ctx *ctx);

/* Process the remaining bytes in the buffer and put result from CTX
   in first 20 bytes following RESBUF.  The result is always in little
   endian byte order, so that a byte-wise output yields to the wanted
   ASCII representation of the message digest.

   IMPORTANT: On some systems it is required that RESBUF be correctly
   aligned for a 32 bits value.  */
extern void *sha_finish_ctx (struct sha_ctx *ctx, void *resbuf);


/* Put result from CTX in first 20 bytes following RESBUF.  The result is
   always in little endian byte order, so that a byte-wise output yields
   to the wanted ASCII representation of the message digest.

   IMPORTANT: On some systems it is required that RESBUF is correctly
   aligned for a 32 bits value.  */
extern void *sha_read_ctx (const struct sha_ctx *ctx, void *resbuf);


/* Compute SHA1 message digest for bytes read from STREAM.  The
   resulting message digest number will be written into the 20 bytes
   beginning at RESBLOCK.  */
extern int sha_stream (FILE *stream, void *resblock);

/* Compute SHA1 message digest for LEN bytes beginning at BUFFER.  The
   result is always in little endian byte order, so that a byte-wise
   output yields to the wanted ASCII representation of the message
   digest.  */
extern void *sha_buffer (const char *buffer, size_t len, void *resblock);

/* Needed for pacman */
char *SHAFile (char *);
void SHAPrint(unsigned char [16]);
