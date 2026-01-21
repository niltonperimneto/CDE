# XmCreatePromptDialog
library call`XmCreatePromptDialog`The SelectionBox PromptDialog convenience creation functionXmCreatePromptDialogcreation functionsXmCreatePromptDialog#include <Xm/SelectioB.h>Widget`XmCreatePromptDialog`WidgetparentStringnameArgListarglistCardinalargcount
## DESCRIPTION


`XmCreatePromptDialog`is a convenience creation function that creates
a DialogShell and an unmanaged SelectionBox child of the DialogShell.
A PromptDialog prompts the user for text input.
It includes a message, a text input region, and three managed buttons.
The default button labels are`OK`,`Cancel`, and`Help`.
An additional button, with`Apply`as the default label, is created
unmanaged; it may be explicitly managed if needed. One additional`WorkArea`child may be added to the SelectionBox after creation.

`XmCreatePromptDialog`forces the value of the SelectionBox resource`XmNdialogType`to`XmDIALOG_PROMPT`.

Use`XtManageChild`to pop up the PromptDialog (passing the SelectionBox
as the widget parameter); use`XtUnmanageChild`to pop it down.

`XmCreatePromptDialog`forces the value of the Shell resource`XmNallowShellResize`to True.

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