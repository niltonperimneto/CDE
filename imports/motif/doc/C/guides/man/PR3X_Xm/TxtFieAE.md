# XmTextFieldCut
library call`XmTextFieldCut`A TextField function that copies the primary selection to the clipboard and deletes the selected textXmTextFieldCutTextField functionsXmTextFieldCut#include <Xm/TextF.h>Boolean`XmTextFieldCut`WidgetwidgetTimetime
## DESCRIPTION


`XmTextFieldCut`copies the primary selected text to the clipboard and
then deletes the primary selected text.
This routine calls the widget's`XmNvalueChangedCallback`and
verification callbacks, either`XmNmodifyVerifyCallback`or`XmNmodifyVerifyCallbackWcs`, or both. If both verification
callback lists are registered, the procedures of the`XmNmodifyVerifyCallback`list are executed first and the resulting
data is passed to the`XmNmodifyVerifyCallbackWcs`callbacks.

This routine calls the`XmNconvertCallback`procedures, possibly
multiple times, with the`selection`member of theXmConvertCallbackStructset to`CLIPBOARD`and with the`parm`member set to`XmMOVE`.
If the transfer is successful, this routine then calls the`XmNconvertCallback`procedures for the`CLIPBOARD`selection
and the`DELETE`target.

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