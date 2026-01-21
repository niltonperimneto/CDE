# XmCreateScrolledList
library call`XmCreateScrolledList`The List ScrolledList convenience creation functionXmCreateScrolledListcreation functionsXmCreateScrolledList#include <Xm/List.h>Widget`XmCreateScrolledList`WidgetparentStringnameArgListarglistCardinalargcount
## DESCRIPTION


`XmCreateScrolledList`creates an instance of a List widget
that is contained within a ScrolledWindow.
The ScrolledWindow parent is created managed.
All ScrolledWindow subarea
widgets are automatically created by this function.
The ID returned by this function is that of the List widget
(not the ScrolledWindow widget).
Use this widget ID for all operations on the List widget.
Use the widget ID of the List widget's parent for all operations
on the ScrolledWindow.
To obtain the ID of the ScrolledWindow widget associated with the List widget,
use the Xt Intrinsics`XtParent`function.
The name of the ScrolledWindow
created by this function is formed by concatenating`SW`onto the end of the`name`specified in the parameter list.

All arguments to either the List or the ScrolledWindow widget can be
specified at creation time using this function.
Changes to initial position and size are sent only to the ScrolledWindow widget.
Other resources are sent to the List or the ScrolledWindow widget as appropriate.
Note that the result of providing the`XmNdestroyCallback`resource in the creation`arglist`is unspecified. The
application should use the`XtAddCallback`function to add
callbacks to the appropriate widget (List or ScrolledWindow) after
creating it.

This function forces the following initial values for ScrolledWindow
resources:

`XmNscrollingPolicy`is set to`XmAPPLICATION_DEFINED`.

`XmNvisualPolicy`is set to`XmVARIABLE`.

`XmNscrollBarDisplayPolicy`is set to`XmSTATIC`.
(No initial value is forced for the List's`XmNscrollBarDisplayPolicy`.)

`XmNshadowThickness`is set to 0 (zero).

* **`parent`** 

Specifies the parent widget ID
* **`name`** 

Specifies the name of the created widget
* **`arglist`** 

Specifies the argument list
* **`argcount`** 

Specifies the number of attribute/value pairs in the argument list
(`arglist`)


For a complete definition of List and its associated resources, see
&cdeman.XmList;.
## RETURN


Returns the List widget ID.
## RELATED


&cdeman.XmList; and &cdeman.XmScrolledWindow;.