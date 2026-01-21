# XmCreateSelectionDialog
library call`XmCreateSelectionDialog`The SelectionBox SelectionDialog convenience creation functionXmCreateSelectionDialogcreation functionsXmCreateSelectionDialog#include <Xm/SelectioB.h>Widget`XmCreateSelectionDialog`WidgetparentStringnameArgListarglistCardinalargcount
## DESCRIPTION


`XmCreateSelectionDialog`is a convenience creation function that creates
a DialogShell and an unmanaged SelectionBox child of the DialogShell.
A SelectionDialog offers the user a choice from a list of alternatives
and gets a selection.
It includes the following:

A scrolling list of alternatives

An editable text field for the selected alternative

Labels for the text field

Four buttons

The default button labels are`OK`,`Cancel`,`Apply`,
and`Help`.
One additional`WorkArea`child may be added to the
SelectionBox after creation.

`XmCreateSelectionDialog`forces the value of the SelectionBox resource`XmNdialogType`to`XmDIALOG_SELECTION`.

`XmCreateSelectionDialog`forces the value of the Shell resource`XmNallowShellResize`to True.

Use`XtManageChild`to pop up the SelectionDialog (passing the
SelectionBox as the widget parameter);
use`XtUnmanageChild`to pop it down.

* **`parent`** 

Specifies the parent widget ID
* **`name`** 

Specifies the name of the created widget
* **`arglist`** 

Specifies the argument list
* **`argcount`** 

Specifies the number of attribute/value pairs in the argument list
(`arglist`)


For a complete definition of SelectionBox and its associated resources, see
&cdeman.XmSelectionBox;.
## RETURN


Returns the SelectionBox widget ID.
## RELATED


&cdeman.XmSelectionBox;.