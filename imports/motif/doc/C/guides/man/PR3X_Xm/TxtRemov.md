# XmTextRemove
library call`XmTextRemove`A Text function that deletes the primary selectionXmTextRemoveText functionsXmTextRemove#include <Xm/Text.h>Boolean`XmTextRemove`Widgetwidget
## DESCRIPTION


`XmTextRemove`deletes the primary selected text.
If there is a selection, this routine also calls the widget's`XmNvalueChangedCallback`and verification callbacks, either`XmNmodifyVerifyCallback`or`XmNmodifyVerifyCallbackWcs`,
or both. If both verification callback lists are registered, the
procedures of the`XmNmodifyVerifyCallback`list are executed
first and the resulting data is passed to the`XmNmodifyVerifyCallbackWcs`callbacks.
This function may also
call the`XmNmotionVerifyCallback`callback.

* **`widget`** 

Specifies the Text widget ID.


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RETURN


This function returns False if the primary selection is NULL or if the`widget`does not own the primary selection. Otherwise, it returns
True.
## RELATED


&cdeman.XmText;.