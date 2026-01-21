# XmCreateMessageBox
library call`XmCreateMessageBox`The MessageBox widget creation functionXmCreateMessageBoxcreation functionsXmCreateMessageBox#include <Xm/MessageB.h>Widget`XmCreateMessageBox`WidgetparentStringnameArgListarglistCardinalargcount
## DESCRIPTION


`XmCreateMessageBox`creates an unmanaged MessageBox.
A MessageBox is used for common interaction tasks, which include giving
information, asking questions, and reporting errors.
It includes an optional symbol, a message, and three buttons.

By default, there is no symbol. The default button labels are`OK`,`Cancel`, and`Help`.

If the parent of the MessageBox is a DialogShell, use`XtManageChild`to
pop up the MessageBox (passing the MessageBox as the widget parameter); use`XtUnmanageChild`to pop it down.

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