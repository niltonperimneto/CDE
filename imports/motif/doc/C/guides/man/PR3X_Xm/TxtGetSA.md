# XmTextGetSelection
library call`XmTextGetSelection`A Text function that retrieves the value of the primary selectionXmTextGetSelectionText functionsXmTextGetSelection#include <Xm/Text.h>char *`XmTextGetSelection`Widgetwidget
## DESCRIPTION


`XmTextGetSelection`retrieves the value of the primary selection. It returns
a NULL pointer if no text is selected in the widget. The application is
responsible for freeing the storage associated with the string by calling`XtFree`.

* **`widget`** 

Specifies the Text widget ID


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RETURN


Returns a character pointer to the string that is associated with the
primary selection.
## RELATED


&cdeman.XmText; and
&cdeman.XmTextGetSelectionWcs;.