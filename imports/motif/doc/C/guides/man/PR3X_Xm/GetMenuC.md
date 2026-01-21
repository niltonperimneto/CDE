# XmGetMenuCursor
library call`XmGetMenuCursor`A function that returns the cursor ID for the current menu cursorXmGetMenuCursor#include <Xm/Xm.h>Cursor`XmGetMenuCursor`Display* display
## DESCRIPTION


`XmGetMenuCursor`queries the menu
cursor currently being used by this client on the specified display and returns
the cursor ID.
This function returns the menu cursor for the default screen of the
display.

`NOTE:``XmGetMenuCursor`is obsolete and exists for compatibility with
previous releases.
Instead of using this function, call`XtGetValues`for the XmScreen
resource`XmNmenuCursor`.

* **`display`** 

Specifies the display whose menu cursor is to be queried

## RETURN


Returns the cursor ID for the current menu cursor or the value None if
a cursor is not yet defined. A cursor will not be defined
if the application makes this call before the client has created any menus
on the specified display.
## RELATED


&cdeman.XmScreen;.