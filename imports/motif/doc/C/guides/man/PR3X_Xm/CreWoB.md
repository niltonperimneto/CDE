# XmCreateWorkingDialog
library call`XmCreateWorkingDialog`The MessageBox WorkingDialog convenience creation functionXmCreateWorkingDialogcreation functionsXmCreateWorkingDialog#include <Xm/MessageB.h>Widget`XmCreateWorkingDialog`WidgetparentStringnameArgListarglistCardinalargcount
## DESCRIPTION


`XmCreateWorkingDialog`is a convenience creation function that
creates a DialogShell and an unmanaged MessageBox child of the DialogShell.
A WorkingDialog informs users that there is a time-consuming
operation in progress and allows them to cancel the
operation.
It includes a symbol, a message, and three buttons.
The default symbol is an hourglass. The default button labels are`OK`,`Cancel`, and`Help`.

Use`XtManageChild`to pop up the WorkingDialog (passing the
MessageBox as the widget parameter);
use`XtUnmanageChild`to pop it down.

`XmCreateWorkingDialog`forces the value of the Shell resource`XmNallowShellResize`to True.

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