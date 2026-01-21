# XmMessageBoxGetChild
library call`XmMessageBoxGetChild`A MessageBox function that is used to access a componentXmMessageBoxGetChildMessageBox functionsXmMessageBoxGetChild#include <Xm/MessageB.h>Widget`XmMessageBoxGetChild`Widgetwidgetunsigned charchild
## DESCRIPTION


`XmMessageBoxGetChild`is used to access a component within a MessageBox.
The parameters given to the function are the MessageBox widget and a value
indicating which component to access.

NOTE: This routine is obsolete and exists for compatibility with previous
releases. Instead of calling`XmMessageBoxGetChild`, you should
call`XtNameToWidget`as described in the &cdeman.XmMessageBox;
reference page.

* **`widget`** 

Specifies the MessageBox widget ID.
* **`child`** 

Specifies a component within the MessageBox. The following are legal values
for this parameter:

`XmDIALOG_CANCEL_BUTTON`

`XmDIALOG_DEFAULT_BUTTON`

`XmDIALOG_HELP_BUTTON`

`XmDIALOG_MESSAGE_LABEL`

`XmDIALOG_OK_BUTTON`

`XmDIALOG_SEPARATOR`

`XmDIALOG_SYMBOL_LABEL`


For a complete definition of MessageBox and its associated resources, see
&cdeman.XmMessageBox;.
## RETURN


Returns the widget ID of the specified MessageBox component.
An application should not assume that the returned widget will be of any
particular class.
## RELATED


&cdeman.XmMessageBox;.