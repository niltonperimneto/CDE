# XmCommandGetChild
library call`XmCommandGetChild`A Command function that is used to access a componentXmCommandGetChildCommand functionsXmCommandGetChild#include <Xm/Command.h>Widget`XmCommandGetChild`Widgetwidgetunsigned charchild
## DESCRIPTION


`XmCommandGetChild`is used to access a component within a Command. The
parameters given to the function are the Command widget and a value
indicating which component to access.

* **`widget`** 

Specifies the Command widget ID.
* **`child`** 

Specifies a component within the Command.
The following values are legal for
this parameter:

`XmDIALOG_COMMAND_TEXT`

`XmDIALOG_PROMPT_LABEL`

`XmDIALOG_HISTORY_LIST`

`XmDIALOG_WORK_AREA`


For a complete definition of Command and its associated resources, see
&cdeman.XmCommand;.
## RETURN


Returns the widget ID of the specified Command component.
An application should not assume that the returned widget will be of any
particular class.
## RELATED


&cdeman.XmCommand;.