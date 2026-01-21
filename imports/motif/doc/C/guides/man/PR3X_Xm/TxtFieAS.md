# XmTextFieldInsertWcs
library call`XmTextFieldInsertWcs`A TextField function that inserts a wide
character string into a TextField widgetXmTextFieldInsertWcsTextField functionsXmTextFieldInsertWcs#include <Xm/TextF.h>void`XmTextFieldInsertWcs`WidgetwidgetXmTextPositionpositionwchar_t *wcstring
## DESCRIPTION


`XmTextFieldInsertWcs`inserts a wide character string into
the TextField widget at a specified location. The character
positions begin at 0 (zero) and are numbered sequentially from the
beginning of the text. For example, to insert a string after
the fourth character, the`position`parameter must be 4.

This routine calls the widget's`XmNvalueChangedCallback`and
verification callbacks, either`XmNmodifyVerifyCallback`or`XmNmodifyVerifyCallbackWcs`, or both. If both verification
callback lists are registered, the procedures of the`XmNmodifyVerifyCallback`list are executed first and the resulting
data is passed to the`XmNmodifyVerifyCallbackWcs`callbacks.
If the`XmNcursorPosition`resource is greater than or is the same value as`position`, the`XmNmotionVerifyCallback`is called.

* **`widget`** 

Specifies the TextField widget ID
* **`position`** 

Specifies the position in the text string where the new character string is
to be inserted
* **`wcstring`** 

Specifies the wide character string value to be added to the TextField widget


For a complete definition of TextField and its associated resources, see
&cdeman.XmTextField;.
## RELATED


&cdeman.XmTextField; and
&cdeman.XmTextFieldInsert;.