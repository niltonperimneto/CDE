# XmTextFieldPasteLink
library call`XmTextFieldPasteLink`A TextField function that inserts a link to the clipboard selectionXmTextFieldPasteLinkTextField functionsXmTextFieldPasteLink#include <Xm/TextF.h>Boolean`XmTextFieldPasteLink`Widgetwidget
## DESCRIPTION


`XmTextFieldPasteLink`inserts a link to the clipboard selection at
the insertion cursor.
This routine calls the widget's`XmNdestinationCallback`procedures
with the`selection`member of theXmDestinationCallbackStructset to`CLIPBOARD`and with the`operation`member set to`XmLINK`.
The TextField widget itself performs no transfers; the`XmNdestinationCallback`procedures are responsible for inserting
the link to the clipboard selection and for taking any related actions.

* **`widget`** 

Specifies the TextField widget ID.


For a complete definition of TextField and its associated resources, see
&cdeman.XmTextField;.
## RETURN


This function returns False if no transfers take place.
Otherwise, it returns True.
## RELATED


&cdeman.XmTextField;.