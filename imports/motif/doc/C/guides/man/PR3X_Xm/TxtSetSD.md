# XmTextSetStringWcs
library call`XmTextSetStringWcs`A Text function that sets a wide character
string valueXmTextSetStringWcsText functionsXmTextSetStringWcs#include <Xm/Text.h>void`XmTextSetStringWcs`Widgetwidgetwchar_t *wcstring
## DESCRIPTION


`XmTextSetStringWcs`sets the wide character string value
of the Text widget. This routine calls the widget's`XmNvalueChangedCallback`and verification callbacks, either`XmNmodifyVerifyCallback`or`XmNmodifyVerifyCallbackWcs`,
or both. If both verification callback lists are registered, the
procedures of the`XmNmodifyVerifyCallback`list are executed
first and the resulting data is passed to the`XmNmodifyVerifyCallbackWcs`callbacks. This function also sets the
insertion cursor position to the beginning of the string and
calls the widget's`XmNmotionVerifyCallback`callbacks.

* **`widget`** 

Specifies the Text widget ID
* **`value`** 

Specifies the wide character string value


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RELATED


&cdeman.XmText; and
&cdeman.XmTextSetString;.