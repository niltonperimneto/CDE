# XmTextCopyLink
library call`XmTextCopyLink`A Text function that copies a link to the primary selection to the clipboardXmTextCopyLinkText functionsXmTextCopyLink#include <Xm/Text.h>Boolean`XmTextCopyLink`WidgetwidgetTimetime
## DESCRIPTION


`XmTextCopyLink`copies a link to the primary selected text to the
clipboard.
This routine calls the`XmNconvertCallback`procedures, possibly
multiple times, with the`selection`member of theXmConvertCallbackStructset to`CLIPBOARD`and with the`parm`member set to`XmLINK`.
The Text widget itself does not copy any links;`XmNconvertCallback`procedures are responsible for copying the link to the clipboard and for
taking any related actions.

* **`widget`** 

Specifies the Text widget ID.
* **`time`** 

Specifies the time of the transfer.
This should be the time of the event which triggered this request.
One source of a valid time stamp is the function`XtLastTimestampProcessed`.


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RETURN


This function returns False if the primary selection is NULL, if the`widget`does not own the primary selection, if the function is
unable to gain ownership of the clipboard selection, or if no data is
placed on the clipboard.
Otherwise, it returns True.
## RELATED


&cdeman.XmText;.