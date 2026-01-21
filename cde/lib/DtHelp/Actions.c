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
/* $XConsortium: Actions.c /main/8 1996/10/30 10:03:40 pascale $ */
/************************************<+>*************************************
 ****************************************************************************
 **
 **   File:        Actions.c
 **
 **   Project:     Display Area Library
 **
 **   Description: This body of code handles the actions for the
 **                Display Area.
 **
 **  (c) Copyright 1987, 1988, 1989, 1990, 1991, 1992 Hewlett-Packard Company
 **
 **  (c) Copyright 1993, 1994 Hewlett-Packard Company
 **  (c) Copyright 1993, 1994 International Business Machines Corp.
 **  (c) Copyright 1993, 1994 Sun Microsystems, Inc.
 **  (c) Copyright 1993, 1994 Novell, Inc.
 **
 **
 ****************************************************************************
 ************************************<+>*************************************/

/*
 * system includes
 */
#include <Xm/Xm.h>
#include <stdlib.h>

/*
 * CanvasEngine
 */
/* #include "CanvasP.h" REMOVED */

/*
 * private includes
 */
#include "ActionsI.h"
#include "CallbacksI.h"
#include "DisplayAreaI.h"
#include "DisplayAreaP.h"

#ifdef NLS16
#endif

/********    Private Function Declarations    ********/
/********    End Private Function Declarations    ********/

/********    End Private Defines             ********/

/********    Private Variable Declarations    ********/

/********    End Private Variable Declarations    ********/

/******************************************************************************
 *                             Private Functions
 ******************************************************************************/
/******************************************************************************
 *                          Semi Public Functions
 *****************************************************************************/
/*****************************************************************************
 * Function: _DtHelpCopyAction
 *
 *    _DtHelpCopyAction - Copy the current info to the clipboard
 *
 *****************************************************************************/
void _DtHelpCopyAction(Widget widget, XEvent *event, String *params,
                       Cardinal *num_params) {
  /* Stubbed: Clipboard support pending Rust reimplementation */
} /* End _DtHelpCopyAction */

/*****************************************************************************
 * Function: _DtHelpDeSelectAll
 *
 *    _DtHelpDeSelectAll - Deselects the information in the widget.
 *
 *****************************************************************************/
void _DtHelpDeSelectAll(Widget widget, XEvent *event, String *params,
                        Cardinal *num_params) {
  Arg args[2];
  XtPointer userData;

  XtSetArg(args[0], XmNuserData, &userData);
  XtGetValues(widget, args, 1);

  if (userData != NULL)
    _DtHelpClearSelection(userData);

} /* End _DtHelpDeSelectAll */

/*****************************************************************************
 * Function: _DtHelpSelectAll
 *
 *    _DtHelpSelectAll - Selects all the information in the widget.
 *
 *****************************************************************************/
void _DtHelpSelectAll(Widget widget, XEvent *event, String *params,
                      Cardinal *num_params) {
  /* Stubbed: Selection support pending Rust reimplementation */
} /* End _DtHelpSelectAll */

/*****************************************************************************
 * Function: _DtHelpActivateLink
 *
 *    _DtHelpSelectAll - Selects all the information in the widget.
 *
 *****************************************************************************/
void _DtHelpActivateLink(Widget widget, XEvent *event, String *params,
                         Cardinal *num_params) {
  /* Stubbed: Link activation pending Rust reimplementation */
} /* End _DtHelpActivateLink */

/*****************************************************************************
 * Function: _DtHelpPageUpOrDown
 *
 *    _DtHelpPageUpOrDown - Selects all the information in the widget.
 *
 *****************************************************************************/
void _DtHelpPageUpOrDown(Widget widget, XEvent *event, String *params,
                         Cardinal *num_params) {
  int keyPressed;
  _DtCvUnit newY;
  _DtCvUnit diff;
  Arg args[2];
  XtPointer userData;

  XtSetArg(args[0], XmNuserData, &userData);
  XtGetValues(widget, args, 1);

  if (userData != NULL) {
    DtHelpDispAreaStruct *pDAS = (DtHelpDispAreaStruct *)userData;

    diff = pDAS->dispUseHeight - pDAS->lineHeight;

    keyPressed = atoi(*params);
    if (keyPressed == 0)
      diff = -diff;

    newY = pDAS->firstVisible + diff;

    /*
     * Is the new Y position too large?
     * If so, adjust.
     */
    if (newY + ((int)pDAS->dispUseHeight) > pDAS->maxYpos)
      newY = pDAS->maxYpos - pDAS->dispUseHeight;

    /*
     * Is the new Y before the begining?
     * If so, zero it.
     */
    if (newY < 0)
      newY = 0;

    if (newY != pDAS->firstVisible) {
      pDAS->firstVisible = newY;
      XtSetArg(args[0], XmNvalue, newY);
      XtSetValues(pDAS->vertScrollWid, args, 1);

      if (pDAS->vScrollNotify)
        (pDAS->vScrollNotify)(pDAS->clientData, pDAS->firstVisible);

      _DtHelpCleanAndDrawWholeCanvas(userData);
    }
  }

} /* End _DtHelpPageUpOrDown */

/*****************************************************************************
 * Function: _DtHelpPageLeftOrRight
 *
 *    _DtHelpPageLeftOrRight - Selects all the information in the widget.
 *
 *****************************************************************************/
void _DtHelpPageLeftOrRight(Widget widget, XEvent *event, String *params,
                            Cardinal *num_params) {
  int keyPressed;
  _DtCvUnit newX;
  _DtCvUnit diff;
  Arg args[2];
  XtPointer userData;

  XtSetArg(args[0], XmNuserData, &userData);
  XtGetValues(widget, args, 1);

  if (userData != NULL) {
    DtHelpDispAreaStruct *pDAS = (DtHelpDispAreaStruct *)userData;

    diff = pDAS->dispUseWidth - ((int)pDAS->charWidth / 10);

    keyPressed = atoi(*params);
    if (keyPressed == 0)
      diff = -diff;

    newX = pDAS->virtualX + diff;

    /*
     * Is the new X position too large?
     * If so, adjust.
     */
    if (newX + ((int)pDAS->dispUseWidth) > pDAS->maxX)
      newX = pDAS->maxX - pDAS->dispUseWidth;

    /*
     * Is the new X before the begining?
     * If so, zero it.
     */
    if (newX < 0)
      newX = 0;

    if (newX != pDAS->virtualX) {
      pDAS->virtualX = newX;
      XtSetArg(args[0], XmNvalue, newX);
      XtSetValues(pDAS->horzScrollWid, args, 1);

      _DtHelpCleanAndDrawWholeCanvas(userData);
    }
  }

} /* End _DtHelpPageLeftOrRight */

/*****************************************************************************
 * Function: _DtHelpNextLink
 *
 *    _DtHelpNextLink - Moves the traversal to the requested hypertext link.
 *
 *****************************************************************************/
void _DtHelpNextLink(Widget widget, XEvent *event, String *params,
                     Cardinal *num_params) {
  /* Stubbed: Link traversal pending Rust reimplementation */
} /* End _DtHelpNextLink */
