# XmCreateTemplateDialog
library call`XmCreateTemplateDialog`A MessageBox TemplateDialog convenience
creation functionXmCreateTemplateDialogcreate functionsXmCreateTemplateDialog#include <Xm/MessageB.h>Widget`XmCreateTemplateDialog`WidgetparentStringnameArgListarglistCardinalargcount
## DESCRIPTION


`XmCreateTemplateDialog`is a convenience creation function
that creates a DialogShell and an unmanaged MessageBox child of
the DialogShell. The MessageBox widget's`XmNdialogType`resource
is set to`XmDIALOG_TEMPLATE`. By default, the TemplateDialog
widget contains only the separator child. You can build
a customized dialog by adding children to the TemplateDialog.

You can create
the standard MessageBox pushbuttons,`Cancel`,`Help`,
and`OK`, by specifying the associated callback
and label string resources. Setting`XmNsymbolPixmap`or`XmNmessageString`creates a symbol or message label.

Use`XtManageChild`to pop up the TemplateDialog (passing the
MessageBox as the widget parameter); use`XtUnmanageChild`to
pop it down.

`XmCreateTemplateDialog`forces the value of the Shell resource`XmNallowShellResize`to True.

* **`parent`** 

Specifies the parent widget ID
* **`name`** 

Specifies the name of the created widget
* **`arglist`** 

Specifies the argument list
* **`argcount`** 

Specifies the number of attribute/value pairs in the argument
list (`arglist`)


For a complete definition of MessageBox and its associated resources,
see &cdeman.XmMessageBox;.
## RETURN


Returns the MessageBox widget ID.
## RELATED


&cdeman.XmMessageBox;.