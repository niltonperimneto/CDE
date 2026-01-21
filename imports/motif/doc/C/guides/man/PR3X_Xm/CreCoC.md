# XmCreateCommandDialog
library call`XmCreateCommandDialog`The Command CommandDialog convenience creation functionXmCreateCommandDialogcreation functionsXmCreateCommandDialog#include <Xm/Command.h>Widget`XmCreateCommandDialog`WidgetparentStringnameArgListarglistCardinalargcount
## DESCRIPTION


`XmCreateCommandDialog`is a convenience function
that creates a DialogShell and an unmanaged Command child of the DialogShell.
A CommandDialog is used for entering commands for processing.

Use`XtManageChild`to pop up the CommandDialog
(passing the Command as the widget parameter);
use`XtUnmanageChild`to pop it down.

`XmCreateCommandDialog`forces the value of the
Shell resource`XmNallowShellResize`to True.

* **`parent`** 

Specifies the parent widget ID
* **`name`** 

Specifies the name of the created widget
* **`arglist`** 

Specifies the argument list
* **`argcount`** 

Specifies the number of attribute/value pairs in the argument list
(`arglist`)


For a complete definition of Command and its associated resources, see
&cdeman.XmCommand;.
## RETURN


Returns the Command widget ID.
## RELATED


&cdeman.XmCommand;.