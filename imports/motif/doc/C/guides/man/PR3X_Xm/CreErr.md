# XmCreateErrorDialog
library call`XmCreateErrorDialog`The MessageBox ErrorDialog convenience creation functionXmCreateErrorDialogcreation functionsXmCreateErrorDialog#include <Xm/MessageB.h>Widget`XmCreateErrorDialog`WidgetparentStringnameArgListarglistCardinalargcount
## DESCRIPTION


`XmCreateErrorDialog`is a convenience creation
function that creates a DialogShell and an unmanaged MessageBox child
of the DialogShell.
An ErrorDialog warns the user of an invalid or potentially
dangerous condition.
It includes a symbol, a message, and three buttons.
The default symbol is an octagon with a diagonal slash. The default
button labels are`OK`,`Cancel`, and`Help`.

Use`XtManageChild`to pop up the ErrorDialog (passing the MessageBox
as the widget parameter); use`XtUnmanageChild`to pop it down.

`XmCreateErrorDialog`forces the value of the Shell resource`XmNallowShellResize`to True.

* **`parent`** 

Specifies the parent widget ID
* **`name`** 

Specifies the name of the created widget
* **`arglist`** 

Specifies the argument list
* **`argcount`** 

Specifies the number of attribute/value pairs in the argument list
(`arglist`)


For a complete definition of MessageBox and its associated resources, see
&cdeman.XmMessageBox;.
## RETURN


Returns the MessageBox widget ID.
## RELATED


&cdeman.XmMessageBox;.