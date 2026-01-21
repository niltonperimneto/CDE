# XmTextClearSelection
library call`XmTextClearSelection`A Text function that clears the primary selectionXmTextClearSelectionText functionsXmTextClearSelection#include <Xm/Text.h>void`XmTextClearSelection`WidgetwidgetTimetime
## DESCRIPTION


`XmTextClearSelection`clears the primary selection in the Text widget.

* **`widget`** 

Specifies the Text widget ID.
* **`time`** 

Specifies the server time at which the selection value is desired. This
should be the time of the event that triggered this request.
One source of a valid time stamp is the function`XtLastTimestampProcessed()`.


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RELATED


&cdeman.XmText;.