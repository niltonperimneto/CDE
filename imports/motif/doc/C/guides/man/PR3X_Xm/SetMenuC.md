# XmSetMenuCursor
library call`XmSetMenuCursor`A function that modifies the menu cursor for a clientXmSetMenuCursor#include <Xm/Xm.h>void`XmSetMenuCursor`Display* displayCursorcursorId
## DESCRIPTION


`XmSetMenuCursor`programmatically modifies the menu cursor for a client;
after the cursor has been created
by the client, this function registers the cursor with the menu system. After
calling this function, the specified cursor is displayed whenever
this client displays a Motif menu on the indicated display. The client can
then specify different cursors on different displays.

This function sets the menu cursor for all screens on the display.`XmSetMenuCursor`is obsolete and exists for compatibility with
previous releases.
Instead of using this function, provide initial values or call`XtSetValues`for the XmScreen resource`XmNmenuCursor`.

* **`display`** 

Specifies the display to which the cursor is to be associated
* **`cursorId`** 

Specifies the`X`cursor ID

## RELATED


&cdeman.XmScreen;.