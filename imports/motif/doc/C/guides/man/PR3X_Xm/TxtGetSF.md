# XmTextGetStringWcs
library call`XmTextGetStringWcs`A Text function that retrieves a copy
of the wide character string value of a Text widgetXmTextGetStringWcsText functionsXmTextGetStringWcs#include <Xm/Text.h>wchar_t *`XmTextGetStringWcs`Widgetwidget
## DESCRIPTION


`XmTextGetStringWcs`retrieves a copy of the wide character string value
of the Text widget. The application is responsible for freeing
the storage associated with the string by calling`XtFree`.

* **`widget`** 

Specifies the Text widget ID


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RETURN


Returns the wide character string value of the Text widget. The
function returns an empty string if the length of the Text widget's
string is 0 (zero).
## RELATED


&cdeman.XmText; and
&cdeman.XmTextGetString;.