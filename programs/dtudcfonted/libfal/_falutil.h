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
/* Xutil.h 1.1 - Fujitsu source for CDEnext    95/11/06 20:32:12 	*/

#ifndef _XUTIL_H_
#define _XUTIL_H_

#include <X11/Xlib.h>
#include <X11/Xresource.h>
#include <X11/Xutil.h>

/***********************************************************

Copyright (c) 1987  X Consortium

... (Copyright text truncated for brevity) ...

******************************************************************/

/* Obsolete definitions removed to avoid conflict with <X11/Xutil.h> */

/****************************************************************
 *
 * Context Management
 *
 ****************************************************************/

/* Associative lookup table return codes */

#define XCSUCCESS 0 /* No error. */
#define XCNOMEM 1   /* Out of memory */
#define XCNOENT 2   /* No entry in table */

typedef int XContext;

#define XUniqueContext() ((XContext)falrmUniqueQuark())
#define XStringToContext(string) ((XContext)falrmStringToQuark(string))

_XFUNCPROTOBEGIN

/* The following declarations are alphabetized. */

extern char *falDefaultString(void);

/* Removed standard X11 function prototypes to avoid conflicts */

extern int falmbTextListToTextProperty(Display * /* display */,
                                       char ** /* list */, int /* count */,
                                       XICCEncodingStyle /* style */,
                                       XTextProperty * /* text_prop_return */
);

extern int falwcTextListToTextProperty(Display * /* display */,
                                       wchar_t ** /* list */, int /* count */,
                                       XICCEncodingStyle /* style */,
                                       XTextProperty * /* text_prop_return */
);

extern void falwcFreeStringList(wchar_t ** /* list */
);

extern Status XTextPropertyToStringList(XTextProperty * /* text_prop */,
                                        char *** /* list_return */,
                                        int * /* count_return */
);

extern int falmbTextPropertyToTextList(Display * /* display */,
                                       XTextProperty * /* text_prop */,
                                       char *** /* list_return */,
                                       int * /* count_return */
);

extern int falwcTextPropertyToTextList(Display * /* display */,
                                       XTextProperty * /* text_prop */,
                                       wchar_t *** /* list_return */,
                                       int * /* count_return */
);

/* More removed standard X11 prototypes */

_XFUNCPROTOEND

#endif /* _XUTIL_H_ */
