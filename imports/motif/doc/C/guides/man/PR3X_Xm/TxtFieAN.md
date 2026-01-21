# XmTextFieldGetString
library call`XmTextFieldGetString`A TextField function that accesses the string valueXmTextFieldGetStringTextField functionsXmTextFieldGetString#include <Xm/TextF.h>char *`XmTextFieldGetString`Widgetwidget
## DESCRIPTION


`XmTextFieldGetString`accesses the string value of the TextField widget. The application
is responsible for freeing the storage associated with the string by
calling`XtFree`.

* **`widget`** 

Specifies the TextField widget ID


For a complete definition of TextField and its associated resources, see
&cdeman.XmTextField;.
## RETURN


Returns a character pointer to the string value of the TextField widget.
This returned value is a copy of the value of the`XmNvalue`resource.
Returns an empty string if the length of the TextField widget's
string is 0 (zero).
## RELATED


&cdeman.XmTextField; and &cdeman.XmTextFieldGetStringWcs;.