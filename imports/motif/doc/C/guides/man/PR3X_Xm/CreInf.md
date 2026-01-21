# XmCreateInformationDialog
library call`XmCreateInformationDialog`The MessageBox InformationDialog convenience creation functionXmCreateInformationDialogcreation functionsXmCreateInformationDialog#include <Xm/MessageB.h>Widget`XmCreateInformationDialog`WidgetparentStringnameArgListarglistCardinalargcount
## DESCRIPTION


`XmCreateInformationDialog`is a convenience creation function that creates
a DialogShell and an unmanaged MessageBox child of the DialogShell.
An InformationDialog gives the user information, such as the status of
an action.
It includes a symbol, a message, and three buttons.
The default symbol is`i`.
The default button labels are`OK`,`Cancel`, and`Help`.

Use`XtManageChild`to pop up the InformationDialog (passing the MessageBox
as the widget parameter); use`XtUnmanageChild`to pop it down.

`XmCreateInformationDialog`forces the value of the Shell resource`XmNallowShellResize`to True.

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