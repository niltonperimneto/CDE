# XmCreateWarningDialog
library call`XmCreateWarningDialog`The MessageBox WarningDialog convenience creation functionXmCreateWarningDialogcreation functionsXmCreateWarningDialog#include <Xm/MessageB.h>Widget`XmCreateWarningDialog`WidgetparentStringnameArgListarglistCardinalargcount
## DESCRIPTION


`XmCreateWarningDialog`is a convenience creation function that
creates a DialogShell and an unmanaged MessageBox child of the DialogShell.
A WarningDialog warns users of action consequences and gives them
a choice of resolutions.
It includes a symbol, a message, and three buttons.
The default symbol is an exclamation point. The default button labels are`OK`,`Cancel`, and`Help`.

Use`XtManageChild`to pop up the WarningDialog (passing the
MessageBox as the widget parameter);
use`XtUnmanageChild`to pop it down.

`XmCreateWarningDialog`forces the value of the Shell resource`XmNallowShellResize`to True.

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