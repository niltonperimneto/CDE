# XmTextGetString
library call`XmTextGetString`A Text function that accesses the string valueXmTextGetStringText functionsXmTextGetString#include <Xm/Text.h>char *`XmTextGetString`Widgetwidget
## DESCRIPTION


`XmTextGetString`accesses the string value of the Text widget. The application
is responsible for freeing the storage associated with the string by
calling`XtFree`.

* **`widget`** 

Specifies the Text widget ID


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RETURN


Returns a character pointer to the string value of the text widget.
This returned value is a copy of the value of the`XmNvalue`resource.
Returns an empty string if the length of the Text widget's string is 0 (zero).
## RELATED


&cdeman.XmText; and
&cdeman.XmTextGetStringWcs;.