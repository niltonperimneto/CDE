# XmTextFieldGetSelectionWcs
library call`XmTextFieldGetSelectionWcs`A TextField function that retrieves the
value of a wide character encoded primary selectionXmTextFieldGetSelection\\%WcsTextField functionsXmTextFieldGetSelection\\%Wcs#include <Xm/TextF.h>wchar_t *`XmTextFieldGetSelectionWcs`Widgetwidget
## DESCRIPTION


`XmTextFieldGetSelectionWcs`retrieves the value of the primary
selection, encoded in a wide character format. It returns a
NULL pointer if no text is selected in the widget. The application
is responsible for freeing the storage associated with the wide
character buffer by calling`XtFree`.

* **`widget`** 

Specifies the TextField widget ID


For a complete definition of TextField and its associated resources, see
&cdeman.XmTextField;.
## RETURN


Returns the wide character string that is associated with the primary
selection in the TextField widget.
## RELATED


&cdeman.XmTextField; and
&cdeman.XmTextFieldGetSelection;.