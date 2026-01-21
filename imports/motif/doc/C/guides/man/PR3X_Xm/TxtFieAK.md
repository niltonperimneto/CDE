# XmTextFieldGetSelection
library call`XmTextFieldGetSelection`A TextField function that retrieves the value of the primary selectionXmTextFieldGetSelectionTextField functionsXmTextFieldGetSelection#include <Xm/TextF.h>char *`XmTextFieldGetSelection`Widgetwidget
## DESCRIPTION


`XmTextFieldGetSelection`retrieves the value of the primary selection. It returns
a NULL pointer if no text is selected in the widget. The application is
responsible for freeing the storage associated with the string by calling`XtFree`.

* **`widget`** 

Specifies the TextField widget ID


For a complete definition of TextField and its associated resources, see
&cdeman.XmTextField;.
## RETURN


Returns a character pointer to the string that is associated with the
primary selection.
## RELATED


&cdeman.XmTextField; and &cdeman.XmTextFieldGetSelectionWcs;.