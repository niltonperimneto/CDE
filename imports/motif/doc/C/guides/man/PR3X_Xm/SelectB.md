# XmSelectionBoxGetChild
library call`XmSelectionBoxGetChild`A SelectionBox function that is used to access a componentXmSelectionBoxGetChildSelectionBox functionsXmSelectionBoxGetChild#include <Xm/SelectioB.h>Widget`XmSelectionBoxGetChild`Widgetwidgetunsigned charchild
## DESCRIPTION


`XmSelectionBoxGetChild`is used to access a component within a
SelectionBox. The parameters given to the function are the SelectionBox
widget and a value indicating which component to access.

NOTE: This routine is obsolete and exists for compatibility with previous
releases. Instead of calling`XmSelectionBoxGetChild`, you should
call`XtNameToWidget`as described in the &cdeman.XmSelectionBox;
reference page.

* **`widget`** 

Specifies the SelectionBox widget ID.
* **`child`** 

Specifies a component within the SelectionBox. The following
values are legal for this parameter:

`XmDIALOG_APPLY_BUTTON`

`XmDIALOG_CANCEL_BUTTON`

`XmDIALOG_DEFAULT_BUTTON`

`XmDIALOG_HELP_BUTTON`

`XmDIALOG_LIST`

`XmDIALOG_LIST_LABEL`

`XmDIALOG_OK_BUTTON`

`XmDIALOG_SELECTION_LABEL`

`XmDIALOG_SEPARATOR`

`XmDIALOG_TEXT`

`XmDIALOG_WORK_AREA`


For a complete definition of SelectionBox and its associated resources, see
&cdeman.XmSelectionBox;.
## RETURN


Returns the widget ID of the specified SelectionBox component.
An application should not assume that the returned widget will be of any
particular class.
## RELATED


&cdeman.XmSelectionBox;.