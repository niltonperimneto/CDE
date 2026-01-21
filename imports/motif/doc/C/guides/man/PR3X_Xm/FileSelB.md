# XmFileSelectionBoxGetChild
library call`XmFileSelectionBoxGetChild`A FileSelectionBox function used to access a componentXmFileSelectionBoxGetChildFileSelectionBox functionsXmFileSelectionBoxGetChild#include <Xm/FileSB.h>Widget`XmFileSelectionBoxGetChild`Widgetwidgetunsigned charchild
## DESCRIPTION


`XmFileSelectionBoxGetChild`is used to access a component within a
FileSelectionBox. The parameters given to the function are the
FileSelectionBox widget and a value indicating which component to access.

NOTE: This routine is obsolete and exists for compatibility with previous
releases. Instead of calling`XmFileSelectionBoxGetChild`, you should
call`XtNameToWidget`as described in the &cdeman.XmFileSelectionBox;
reference page.

* **`widget`** 

Specifies the FileSelectionBox widget ID.
* **`child`** 

Specifies a component within the FileSelectionBox. The following are legal
values for this parameter:

`XmDIALOG_APPLY_BUTTON`

`XmDIALOG_CANCEL_BUTTON`

`XmDIALOG_DEFAULT_BUTTON`

`XmDIALOG_DIR_LIST`

`XmDIALOG_DIR_LIST_LABEL`

`XmDIALOG_FILTER_LABEL`

`XmDIALOG_FILTER_TEXT`

`XmDIALOG_HELP_BUTTON`

`XmDIALOG_LIST`

`XmDIALOG_LIST_LABEL`

`XmDIALOG_OK_BUTTON`

`XmDIALOG_SELECTION_LABEL`

`XmDIALOG_SEPARATOR`

`XmDIALOG_TEXT`

`XmDIALOG_WORK_AREA`


For a complete definition of FileSelectionBox and its associated resources, see
&cdeman.XmFileSelectionBox;.
## RETURN


Returns the widget ID of the specified FileSelectionBox component.
An application should not assume that the returned widget will be of any
particular class.
## RELATED


&cdeman.XmFileSelectionBox;.