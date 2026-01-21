# XmCreateScrolledText
library call`XmCreateScrolledText`The Text ScrolledText convenience creation functionXmCreateScrolledTextcreation functionsXmCreateScrolledText#include <Xm/Text.h>Widget`XmCreateScrolledText`WidgetparentStringnameArgListarglistCardinalargcount
## DESCRIPTION


`XmCreateScrolledText`creates an instance of a Text widget
that is contained within a ScrolledWindow.
The ScrolledWindow parent is created managed.
All ScrolledWindow subarea
widgets are automatically created by this function.
The ID returned by this function is that of the Text widget
(not the ScrolledWindow widget).
Use this widget ID for all operations on the Text widget.
Use the widget ID of the Text widget's parent for
all operations on the ScrolledWindow.
To obtain the ID of the ScrolledWindow widget associated with the
Text widget, use the Xt Intrinsics`XtParent`function.
The name of the ScrolledWindow created by this function is formed by
concatenating the letters`SW`onto the end of the`name`specified in the parameter list.

The Text widget defaults to single-line text edit; therefore, no ScrollBars
are displayed. The Text resource`XmNeditMode`must be set to`XmMULTI_LINE_EDIT`to display the ScrollBars.
The results of placing a Text widget inside a ScrolledWindow when the
Text's`XmNeditMode`is`XmSINGLE_LINE_EDIT`are undefined.

All arguments to either the Text or the ScrolledWindow widget can be
specified at creation time with this function.
Changes to initial position and size are sent only to the ScrolledWindow
widget.
Other resources are sent to the Text or the ScrolledWindow widget as
appropriate.
Note that the result of providing the`XmNdestroyCallback`resource in the creation`arglist`is unspecified. The
application should use the`XtAddCallback`function to add
callbacks to the appropriate widget (Text or ScrolledWindow) after
creating it.

This function forces the following initial values for ScrolledWindow
resources:

`XmNscrollingPolicy`is set to`XmAPPLICATION_DEFINED`.

`XmNvisualPolicy`is set to`XmVARIABLE`.

`XmNscrollBarDisplayPolicy`is set to`XmSTATIC`.

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


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RETURN


Returns the Text widget ID.
## RELATED


&cdeman.XmScrolledWindow; and &cdeman.XmText;.