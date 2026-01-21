# XmTextFieldSetString
library call`XmTextFieldSetString`A TextField function that sets the string valueXmTextFieldSetStringTextField functionsXmTextFieldSetString#include <Xm/TextF.h>void`XmTextFieldSetString`Widgetwidgetchar* value
## DESCRIPTION


`XmTextFieldSetString`sets the string value of the TextField widget.
This routine calls the widget's`XmNvalueChangedCallback`and
verification callbacks, either`XmNmodifyVerifyCallback`or`XmNmodifyVerifyCallbackWcs`, or both. If both verification
callback lists are registered, the procedures of the`XmNmodifyVerifyCallback`list are executed first and the resulting
data is passed to the`XmNmodifyVerifyCallbackWcs`callbacks.
It also sets the insertion cursor position to the beginning of the
string and calls the widget's`XmNmotionVerifyCallback`callbacks.

* **`widget`** 

Specifies the TextField widget ID
* **`value`** 

Specifies the character pointer to the string value and places
the string into the text edit window


For a complete definition of TextField and its associated resources, see
&cdeman.XmTextField;.
## RELATED


&cdeman.XmTextField; and
&cdeman.XmTextFieldSetStringWcs;.