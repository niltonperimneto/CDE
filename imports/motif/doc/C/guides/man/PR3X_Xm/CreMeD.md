# XmCreateMessageDialog
library call`XmCreateMessageDialog`The MessageBox MessageDialog convenience creation functionXmCreateMessageDialogcreation functionsXmCreateMessageDialog#include <Xm/MessageB.h>Widget`XmCreateMessageDialog`WidgetparentStringnameArgListarglistCardinalargcount
## DESCRIPTION


`XmCreateMessageDialog`is a convenience creation function that creates
a DialogShell and an unmanaged MessageBox child of the DialogShell.
A MessageDialog is used for common interaction tasks, which include giving
information, asking questions, and reporting errors.
It includes a symbol, a message, and three buttons.
By default, there is no symbol. The default button labels are`OK`,`Cancel`, and`Help`.

Use`XtManageChild`to pop up the MessageDialog (passing the MessageBox
as the widget parameter); use`XtUnmanageChild`to pop it down.

`XmCreateMessageDialog`forces the value of the Shell resource`XmNallowShellResize`to True.

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