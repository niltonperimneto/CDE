# XmTextFieldGetStringWcs
library call`XmTextFieldGetStringWcs`A TextField function that retrieves a copy
of the wide character string value of a TextField widgetXmTextFieldGetStringWcsTextField functionsXmTextFieldGetStringWcs#include <Xm/TextF.h>wchar_t *`XmTextFieldGetStringWcs`Widgetwidget
## DESCRIPTION


`XmTextFieldGetStringWcs`retrieves a copy of the wide character
string value of the TextField widget. The application is responsible
for freeing the storage associated with the string by calling`XtFree`.

* **`widget`** 

Specifies the TextField widget ID


For a complete definition of TextField and its associated resources, see
&cdeman.XmTextField;.
## RETURN


Returns the wide character string value of the TextField widget. The
function returns an empty string if the length of the TextField
widget's string is 0 (zero).
## RELATED


&cdeman.XmTextField; and
&cdeman.XmTextFieldGetString;.