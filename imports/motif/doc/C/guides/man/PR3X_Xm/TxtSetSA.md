# XmTextSetSelection
library call`XmTextSetSelection`A Text function that sets the primary selection of the textXmTextSetSelectionText functionsXmTextSetSelection#include <Xm/Text.h>void`XmTextSetSelection`WidgetwidgetXmTextPositionfirstXmTextPositionlastTimetime
## DESCRIPTION


`XmTextSetSelection`sets the primary selection of the text in the
widget.
It also sets the insertion cursor position to the last position of the
selection and calls the widget's`XmNmotionVerifyCallback`callbacks.

* **`widget`** 

Specifies the Text widget ID
* **`first`** 

Marks the first character position of the text to be selected
* **last** 

Marks the last position of the text to be selected
* **`time`** 

Specifies the time at which the selection value is desired. This should
be the same as the time of the event that triggered this request.
request. One source of a valid time stamp is the function`XtLastTimestampProcessed`.


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RELATED


&cdeman.XmText;.