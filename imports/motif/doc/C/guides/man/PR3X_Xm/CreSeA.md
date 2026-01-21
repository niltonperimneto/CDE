# XmCreateSelectionBox
library call`XmCreateSelectionBox`The SelectionBox widget creation functionXmCreateSelectionBoxcreation functionsXmCreateSelectionBox#include <Xm/SelectioB.h>Widget`XmCreateSelectionBox`WidgetparentStringnameArgListarglistCardinalargcount
## DESCRIPTION


`XmCreateSelectionBox`creates an unmanaged SelectionBox.
A SelectionBox is used to get a selection
from a list of alternatives from the user and
includes the following:

A scrolling list of alternatives

An editable text field for the selected alternative

Labels for the list and text field

Three or four buttons

The default button labels are`OK`,`Cancel`, and`Help`.
By default, an`Apply`button is also created. If the parent of the
SelectionBox is a DialogShell, it is managed; otherwise it is unmanaged.
Additional work area children may be added to the SelectionBox after
creation.

* **`parent`** 

Specifies the parent widget ID
* **`name`** 

Specifies the name of the created widget
* **`arglist`** 

Specifies the argument list
* **`argcount`** 

Specifies the number of attribute/value pairs in the argument list
(`arglist`)


For a complete definition of SelectionBox
and its associated resources, see &cdeman.XmSelectionBox;.
## RETURN


Returns the SelectionBox widget ID.
## RELATED


&cdeman.XmSelectionBox;.