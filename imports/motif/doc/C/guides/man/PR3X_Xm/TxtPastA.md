# XmTextPaste
library call`XmTextPaste`A Text function that inserts the clipboard selectionXmTextPasteText functionsXmTextPaste#include <Xm/Text.h>Boolean`XmTextPaste`Widgetwidget
## DESCRIPTION


`XmTextPaste`inserts the clipboard selection at the insertion cursor
of the destination widget.
If`XmNpendingDelete`is True and the insertion cursor is inside
the current selection, the clipboard selection replaces the selected text.

This routine calls the widget's`XmNvalueChangedCallback`and
verification callbacks, either`XmNmodifyVerifyCallback`or`XmNmodifyVerifyCallbackWcs`, or both. If both verification
callback lists are registered, the procedures of the`XmNmodifyVerifyCallback`list are executed first and the
resulting data is passed to the`XmNmodifyVerifyCallbackWcs`callbacks.
If the`XmNcursorPosition`resource is greater than or is the same value as
the position where the selection is to be inserted, the`XmNmotionVerifyCallback`is called.

This routine calls the widget's`XmNdestinationCallback`procedures
with the`selection`member of theXmDestinationCallbackStructset to`CLIPBOARD`and with the`operation`member set to`XmCOPY`.

* **`widget`** 

Specifies the Text widget ID.


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RETURN


This function returns False if no transfers take place.
Otherwise, it returns True.
## RELATED


&cdeman.XmText;.