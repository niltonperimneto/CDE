# DtEditorGetMessageTextFieldID
library call`DtEditorGetMessageTextFieldID`retrieve the widget ID of the message text field in the DtEditor status lineDtEditorGetMessageTextFieldIDDtEditor functionsDtEditorGetMessageTextFieldID#include <Dt/Editor.h>Widget`DtEditorGetMessageTextFieldID`Widgetwidget
## DESCRIPTION


The`DtEditorGetMessageTextFieldID`function returns the widget ID of the Motif Text Field located in the
status area of a DtEditor widget.
By setting the`XmNvalue`or`XmNvalueWcs`resource of this widget, an application can
display feedback messages for the user.
If the application does not use the message field,
the message field can be unmanaged by calling`XtUnmanageWidget`(3) with this widget ID.

The`widget`argument specifies the DtEditor widget ID.

For a complete definition of the DtEditor widget and its associated resources, see
&cdeman.DtEditor;.
## RETURN VALUE


Upon successful completion, the`DtEditorGetMessageTextFieldID`function returns the ID of the text field widget; otherwise, it returns`NULL`.
## SEE ALSO


&cdeman.Dt.Editor.h;, &cdeman.DtEditor;, &cdeman.XmTextField;.