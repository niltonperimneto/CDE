# XmTextGetSelectionWcs
library call`XmTextGetSelectionWcs`A Text function that retrieves the
value of a wide character encoded primary selectionXmTextGetSelectionWcsText functionsXmTextGetSelectionWcs#include <Xm/Text.h>wchar_t *`XmTextGetSelectionWcs`Widgetwidget
## DESCRIPTION


`XmTextGetSelectionWcs`retrieves the value of the primary selection
that is encoded in a wide character format. It returns a NULL pointer
if no text is selected in the widget. The application is responsible
for freeing the storage associated with the wide character buffer by
calling`XtFree`.

* **`widget`** 

Specifies the Text widget ID


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RETURN


Returns the wide character string that is associated with the
primary selection in the Text widget.
## RELATED


&cdeman.XmText; and
&cdeman.XmTextGetSelection;.