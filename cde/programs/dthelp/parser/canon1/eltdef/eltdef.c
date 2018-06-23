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
/* $XConsortium: eltdef.c /main/3 1995/11/08 09:27:27 rswiston $ */
/*
              Copyright 1986 Tandem Computers Incorporated.
This product and information is proprietary of Tandem Computers Incorporated.
                   Copyright (c) 1986, 1987, 1988, 1989 Hewlett-Packard Co.
*/

/* Eltdef.c contains the main program for ELTDEF */

#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include "basic.h"
#include "trie.h"

#define M_DTDDEF
#include "dtd.h"

#include "context.h"
#include "delim.h"
#define ELTDEF
#include "eltdef.h"
#define M_ENTDEF
#include "entity.h"

/* Main program */
int main(argc, argv)
  int argc ;
  char **argv ;
  {
    int m_token ;

    if (argc > 2) {
      if (*argv[2] == '-') {
        if (strchr(argv[2], 'a')) m_malftrace = TRUE ;
        if (strchr(argv[2], 'A')) m_malftrace = TRUE ;
        if (strchr(argv[2], 'd')) debug = TRUE ;
        if (strchr(argv[2], 'D')) debug = TRUE ;
        if (strchr(argv[2], 'h')) m_heapchk = TRUE ;
        if (strchr(argv[2], 'H')) m_heapchk = TRUE ;
        if (strchr(argv[2], 's')) scantrace = TRUE ;
        if (strchr(argv[2], 'S')) scantrace = TRUE ;
        }
      else
        fprintf(stderr,
     "****Starting with MARKUP 2.0, ELTDEF no longer writes a statistics file"
         ) ;
      }
    if (argc < 2) {
      fprintf(stderr, "**** Specify interface file ****\n") ;
      exit(TRUE) ;
      }
    snprintf(iffile, sizeof(iffile), "%s", argv[1]) ;
    initialize() ;
    while (TRUE) {
      m_token = scan() ;
      if (m_token == ELT) restart = RELEMENT ;
      m_prevcon = curcon ;
      curcon = m_newcon(m_prevcon - 1, m_token - 1) ;
      if (scantrace)
        printf(
       "Main: m_prevcon=%d,m_token=%d,curcon=%d,textchar='%c'(%d), line=%d\n",
        m_prevcon, m_token, curcon, (char)textchar, (int)textchar, m_line) ;
#include "case.c"
      if (m_token == ENDFILE) break ;
      if (! curcon) {
        m_error("Parsing table error") ;
        break ;
        }
      }
    done() ;
    return 0;
    }
