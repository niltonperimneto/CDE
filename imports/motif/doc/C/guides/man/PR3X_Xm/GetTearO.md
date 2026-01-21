# XmGetTearOffControl
library call`XmGetTearOffControl`A RowColumn function that obtains the widget ID for the tear-off control in a menuXmGetTearOffControlRowColumn functionsXmGetTearOffControl#include <Xm/RowColumn.h>Widget`XmGetTearOffControl`Widgetmenu
## DESCRIPTION


`XmGetTearOffControl`provides the application with the means for
obtaining the widget ID of the internally created tear-off control in a
tear-off menu.

RowColumn creates a tear-off control for a PulldownMenu or PopupMenu
when the`XmNtearOffModel`resource is initialized or set to`XmTEAR_OFF_ENABLED`.
The tear-off control is a widget that appears as the first element in
the menu.
The user tears off the menu by means of mouse or keyboard events in the
tear-off control.

The tear-off control has Separator-like behavior.
Once the application has obtained the widget ID of the tear-off control,
it can set resources to specify the appearance of the control.
The application or user can also set these resources in a resource file
by using the name of the control, which is`TearOffControl`.
For a list of the resources the application or user can set, see
&cdeman.XmRowColumn;.

* **`menu`** 

Specifies the widget ID of the RowColumn PulldownMenu or PopupMenu


For more information on tear-off menus and a complete definition of
RowColumn and its associated resources, see &cdeman.XmRowColumn;.
## RETURN


Returns the widget ID for the tear-off control, or NULL if no tear-off
control exists.
An application should not assume that the returned widget will be of any
particular class.
## RELATED


&cdeman.XmRowColumn;.