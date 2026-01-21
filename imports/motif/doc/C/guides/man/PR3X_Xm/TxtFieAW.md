# XmTextFieldRemove
library call`XmTextFieldRemove`A TextField function that deletes the primary selectionXmTextFieldRemoveTextField functionsXmTextFieldRemove#include <Xm/TextF.h>Boolean`XmTextFieldRemove`Widgetwidget
## DESCRIPTION


`XmTextFieldRemove`deletes the primary selected text.
If there is a selection, this routine also calls the widget's`XmNvalueChangedCallback`and verification callbacks, either`XmNmodifyVerifyCallback`or`XmNmodifyVerifyCallbackWcs`,
or both. If both verification callback lists are registered, the
procedures of the`XmNmodifyVerifyCallback`list are executed
first and the resulting data is passed to the`XmNmodifyVerifyCallbackWcs`callbacks.
This function may also
call the`XmNmotionVerifyCallback`callback.

* **`widget`** 

Specifies the TextField widget ID.


For a complete definition of TextField and its associated resources, see
&cdeman.XmTextField;.
## RETURN


This function returns False if the primary selection is NULL or if the`widget`does not own the primary selection. Otherwise, it returns
True.
## RELATED


&cdeman.XmTextField;.