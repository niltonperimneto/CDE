# XmCreateFileSelectionBox
library call`XmCreateFileSelectionBox`The FileSelectionBox widget creation functionXmCreateFileSelection\\%Boxcreation functionsXmCreateFileSelection\\%Box#include <Xm/FileSB.h>Widget`XmCreateFileSelectionBox`WidgetparentStringnameArgListarglistCardinalargcount
## DESCRIPTION


`XmCreateFileSelectionBox`creates an unmanaged FileSelectionBox.
A FileSelectionBox is used to select a file
and includes the following:

An editable text field for the directory mask

A scrolling list of filenames

An editable text field for the selected file

Labels for the list and text fields

Four buttons

The default button labels are`OK`,`Filter`,`Cancel`, and`Help`.
Additional work area children may be added to the FileSelectionBox after
creation. FileSelectionBox inherits the layout functionality provided
by SelectionBox for any additional work area children.

If the parent of the FileSelectionBox is a DialogShell, use`XtManageChild`to pop up the FileSelectionDialog (passing the
FileSelectionBox as the widget parameter);
use`XtUnmanageChild`to pop it down.

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