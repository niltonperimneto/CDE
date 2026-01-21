# XmTextFieldSetStringWcs
library call`XmTextFieldSetStringWcs`A TextField function that sets a wide
character string valueXmTextFieldSetStringWcsTextField functionsXmTextFieldSetStringWcs#include <Xm/TextF.h>void`XmTextFieldSetStringWcs`Widgetwidgetwchar_t *wcstring
## DESCRIPTION


`XmTextFieldSetStringWcs`sets the wide character string value
of the TextField widget. This routine calls the widget's`XmNvalueChangedCallback`and verification callbacks, either`XmNmodifyVerifyCallback`or`XmNmodifyVerifyCallbackWcs`,
or both. If both verification callback lists are registered, the
procedures of the`XmNmodifyVerifyCallback`list are executed
first and the resulting data is passed to the`XmNmodifyVerifyCallbackWcs`callbacks. It also sets the insertion
cursor position to the beginning of the string and calls the widget's`XmNmotionVerifyCallback`callbacks.

* **`widget`** 

Specifies the TextField widget ID
* **`wcstring`** 

Specifies the wide character string value


For a complete definition of TextField and its associated resources, see
&cdeman.XmTextField;.
## RELATED


&cdeman.XmTextField; and
&cdeman.XmTextFieldSetString;.