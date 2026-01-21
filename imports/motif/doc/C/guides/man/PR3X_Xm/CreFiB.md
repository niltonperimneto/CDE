# XmCreateFileSelectionDialog
library call`XmCreateFileSelectionDialog`The FileSelectionBox FileSelectionDialog convenience creation functionXmCreateFileSelection\\%Dialogcreation functionsXmCreateFileSelection\\%Dialog#include <Xm/FileSB.h>Widget`XmCreateFileSelectionDialog`WidgetparentStringnameArgListarglistCardinalargcount
## DESCRIPTION


`XmCreateFileSelectionDialog`is a convenience creation function that
creates a DialogShell and an unmanaged FileSelectionBox child of the
DialogShell.
A FileSelectionDialog selects a file.
It includes the following:

An editable text field for the directory mask

A scrolling list of filenames

An editable text field for the selected file

Labels for the list and text fields

Four buttons

The default button labels are`OK`,`Filter`,`Cancel`, and`Help`. One additional`WorkArea`child may be added to the FileSelectionBox after creation.

Use`XtManageChild`to pop up the FileSelectionDialog (passing the
FileSelectionBox
as the widget parameter); use`XtUnmanageChild`to pop it down.

`XmCreateFileSelectionDialog`forces the value of the Shell resource`XmNallowShellResize`to True.

* **`parent`** 

Specifies the parent widget ID
* **`name`** 

Specifies the name of the created widget
* **`arglist`** 

Specifies the argument list
* **`argcount`** 

Specifies the number of attribute/value pairs in the argument list
(`arglist`)


For a complete definition of FileSelectionBox and its associated resources, see
&cdeman.XmFileSelectionBox;.
## RETURN


Returns the FileSelectionBox widget ID.
## RELATED


&cdeman.XmFileSelectionBox;.