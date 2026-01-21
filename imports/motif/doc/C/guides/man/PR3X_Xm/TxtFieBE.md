# XmTextFieldSetSelection
library call`XmTextFieldSetSelection`A TextField function that sets the primary selection of the textXmTextFieldSetSelectionTextField functionsXmTextFieldSetSelection#include <Xm/TextF.h>void`XmTextFieldSetSelection`WidgetwidgetXmTextPositionfirstXmTextPositionlastTimetime
## DESCRIPTION


`XmTextFieldSetSelection`sets the primary selection of the text in
the widget.
It also sets the insertion cursor position to the last position of the
selection and calls the widget's`XmNmotionVerifyCallback`callbacks.`XmTextFieldSetSelection`always generates an`XmNgainPrimaryCallbackunless it fails to take ownership of the primary text selection.

* **`widget`** 

Specifies the TextField widget ID
* **`first`** 

Marks the first character position of the text to be selected
* **last** 

Marks the last position of the text to be selected
* **`time`** 

Specifies the time at which the selection value is desired. This should
be the same as the time of the event that triggered this
request. One source of a valid time stamp is the function`XtLastTimestampProcessed`.


For a complete definition of TextField and its associated resources, see
&cdeman.XmTextField;.
## RELATED


&cdeman.XmTextField;.