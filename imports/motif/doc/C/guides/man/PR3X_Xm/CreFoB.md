# XmCreateFormDialog
library call`XmCreateFormDialog`A Form FormDialog convenience creation functionXmCreateFormDialogcreation functionsXmCreateFormDialog#include <Xm/Form.h>Widget`XmCreateFormDialog`WidgetparentStringnameArgListarglistCardinalargcount
## DESCRIPTION


`XmCreateFormDialog`is a convenience creation function that creates
a DialogShell and an unmanaged Form child of the DialogShell.
A FormDialog is used for interactions not supported by the standard dialog
set.
This function
does not automatically create any labels, buttons, or other dialog components.
Such components should be added by the application after the FormDialog is
created.

Use`XtManageChild`to pop up the FormDialog (passing the Form as the
widget parameter); use`XtUnmanageChild`to pop it down.

`XmCreateFormDialog`forces the value of the Shell resource`XmNallowShellResize`to True.

* **`parent`** 

Specifies the parent widget ID
* **`name`** 

Specifies the name of the created widget
* **`arglist`** 

Specifies the argument list
* **`argcount`** 

Specifies the number of attribute/value pairs in the argument list
(`arglist`)


For a complete definition of Form and its associated resources, see
&cdeman.XmForm;.
## RETURN


Returns the Form widget ID.
## RELATED


&cdeman.XmForm;.