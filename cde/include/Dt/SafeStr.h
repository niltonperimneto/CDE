/*
 * CDE - Common Desktop Environment
 *
 * Copyright (c) 1993-2012, The Open Group. All rights reserved.
 *
 * These libraries and programs are free software; you can
 * redistribute them and/or modify them under the terms of the GNU
 * Lesser General Public License as published by the Free Software
 * Foundation; either version 2 of the License, or (at your option)
 * any later version.
 *
 * These libraries and programs are distributed in the hope that
 * they will be useful, but WITHOUT ANY WARRANTY; without even the
 * implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
 * PURPOSE. See the GNU Lesser General Public License for more
 * details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with these libraries and programs; if not, write
 * to the Free Software Foundation, Inc., 51 Franklin Street, Fifth
 * Floor, Boston, MA 02110-1301 USA
 */
/*
 * SafeStr.h - Safe string functions portability shim
 *
 * Provides strlcpy() and strlcat() for systems that lack them
 * (e.g., Linux glibc). These functions are safer alternatives to
 * strcpy() and strcat() as they guarantee NUL-termination and
 * prevent buffer overflows.
 *
 * Usage:
 *   #include <Dt/SafeStr.h>
 *   strlcpy(dest, src, sizeof(dest));
 *   strlcat(dest, src, sizeof(dest));
 */

#ifndef _Dt_SafeStr_h
#define _Dt_SafeStr_h

#include <stddef.h>
#include <string.h>

#ifdef __cplusplus
extern "C" {
#endif

/*
 * Check for native strlcpy/strlcat support.
 * BSD systems and glibc 2.38+ have these functions.
 */
#if defined(__GLIBC__)
#if __GLIBC__ > 2 || (__GLIBC__ == 2 && __GLIBC_MINOR__ >= 38)
#define HAVE_STRLCPY 1
#define HAVE_STRLCAT 1
#endif
#elif defined(__FreeBSD__) || defined(__OpenBSD__) || defined(__NetBSD__) ||   \
    defined(__DragonFly__) || defined(__APPLE__)
#define HAVE_STRLCPY 1
#define HAVE_STRLCAT 1
#endif

#ifndef HAVE_STRLCPY
/*
 * strlcpy - Safe string copy
 *
 * Copy at most (dstsize - 1) characters from src to dst,
 * NUL-terminating the result. Returns strlen(src).
 */
static inline size_t strlcpy(char *dst, const char *src, size_t dstsize) {
  const char *s = src;
  size_t n = dstsize;

  /* Copy as many bytes as will fit */
  if (n != 0) {
    while (--n != 0) {
      if ((*dst++ = *s++) == '\0')
        break;
    }
  }

  /* Not enough room in dst, add NUL and traverse rest of src */
  if (n == 0) {
    if (dstsize != 0)
      *dst = '\0'; /* NUL-terminate dst */
    while (*s++)
      ;
  }

  return (size_t)(s - src - 1); /* count does not include NUL */
}
#endif /* !HAVE_STRLCPY */

#ifndef HAVE_STRLCAT
/*
 * strlcat - Safe string concatenation
 *
 * Appends src to string dst of size dstsize (unlike strncat, dstsize
 * is the full size of dst, not space left). At most dstsize-1 characters
 * will be copied. Always NUL terminates (unless dstsize <= strlen(dst)).
 * Returns strlen(src) + MIN(dstsize, strlen(initial dst)).
 */
static inline size_t strlcat(char *dst, const char *src, size_t dstsize) {
  char *d = dst;
  const char *s = src;
  size_t n = dstsize;
  size_t dlen;

  /* Find the end of dst and adjust bytes left but don't go past end */
  while (n-- != 0 && *d != '\0')
    d++;
  dlen = (size_t)(d - dst);
  n = dstsize - dlen;

  if (n == 0)
    return dlen + strlen(s);

  while (*s != '\0') {
    if (n != 1) {
      *d++ = *s;
      n--;
    }
    s++;
  }
  *d = '\0';

  return dlen + (size_t)(s - src); /* count does not include NUL */
}
#endif /* !HAVE_STRLCAT */

#ifdef __cplusplus
}
#endif

#endif /* _Dt_SafeStr_h */
