# XmTextPasteLink
library call`XmTextPasteLink`A Text function that inserts a link to the clipboard selectionXmTextPasteLinkText functionsXmTextPasteLink#include <Xm/Text.h>Boolean`XmTextPasteLink`Widgetwidget
## DESCRIPTION


`XmTextPasteLink`inserts a link to the clipboard selection at the
insertion cursor.
This routine calls the widget's`XmNdestinationCallback`procedures
with the`selection`member of theXmDestinationCallbackStructset to`CLIPBOARD`and with the`operation`member set to`XmLINK`.
The Text widget itself performs no transfers; the`XmNdestinationCallback`procedures are responsible for inserting
the link to the clipboard selection and for taking any related actions.

* **`widget`** 

Specifies the Text widget ID.


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RETURN


This function returns False if no transfers take place.
Otherwise, it returns True.
## RELATED


&cdeman.XmText;.