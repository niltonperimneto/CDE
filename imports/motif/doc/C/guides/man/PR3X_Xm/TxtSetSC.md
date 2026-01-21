# XmTextSetString
library call`XmTextSetString`A Text function that sets the string valueXmTextSetStringText functionsXmTextSetString#include <Xm/Text.h>void`XmTextSetString`Widgetwidgetchar* value
## DESCRIPTION


`XmTextSetString`sets the string value of the Text widget.
This routine calls the widget's`XmNvalueChangedCallback`and
verification callbacks, either`XmNmodifyVerifyCallback`or`XmNmodifyVerifyCallbackWcs`, or both. If both verification
callback lists are registered, the procedures of the`XmNmodifyVerifyCallback`list are executed first and the resulting
data is passed to the`XmNmodifyVerifyCallbackWcs`callbacks.
This function also sets the insertion cursor position to the beginning of the
string and calls the widget's`XmNmotionVerifyCallback`callbacks.

* **`widget`** 

Specifies the Text widget ID
* **`value`** 

Specifies the character pointer to the string value and places the
string into the text edit window


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RELATED


&cdeman.XmText; and
&cdeman.XmTextSetStringWcs;.