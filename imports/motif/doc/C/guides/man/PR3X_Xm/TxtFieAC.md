# XmTextFieldCopy
library call`XmTextFieldCopy`A TextField function that copies the primary selection to the clipboardXmTextFieldCopyTextField functionsXmTextFieldCopy#include <Xm/TextF.h>Boolean`XmTextFieldCopy`WidgetwidgetTimetime
## DESCRIPTION


`XmTextFieldCopy`copies the primary selected text to the clipboard.

This routine calls the`XmNconvertCallback`procedures, possibly
multiple times, with the`selection`member of theXmConvertCallbackStructset to`CLIPBOARD`and with the`parm`member set to`XmCOPY`.

* **`widget`** 

Specifies the TextField widget ID.
* **`time`** 

Specifies the time at which the selection value is to be modified. This
should be the time of the event that triggered this request.


For a complete definition of TextField and its associated resources, see
&cdeman.XmTextField;.
## RETURN


This function returns False if the primary selection is NULL, if the`widget`does not own the primary selection, if the function is
unable to gain ownership of the clipboard selection, or if no data is
placed on the clipboard.
Otherwise, it returns True.
## RELATED


&cdeman.XmTextField;.