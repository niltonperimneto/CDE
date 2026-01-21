# XmTextFieldReplaceWcs
library call`XmTextFieldReplaceWcs`A TextField function that replaces part of a
wide character string in a TextField widgetXmTextFieldReplaceWcsTextField functionsXmTextFieldReplaceWcs#include <Xm/TextF.h>void`XmTextFieldReplaceWcs`WidgetwidgetXmTextPositionfrom_posXmTextPositionto_poswchar_t *wcstring
## DESCRIPTION


`XmTextFieldReplaceWcs`replaces part of the wide character string
in the TextField widget. The character positions begin at 0 (zero) and are
numbered sequentially from the beginning of the text.

An example text replacement would be to replace
the second and third characters in the text string. To accomplish this,
the parameter`from_pos`must be 1 and`to_pos`must be 3. To
insert a string after the fourth character, both parameters,`from_pos`and`to_pos`, must be 4.

This routine calls the widget's`XmNvalueChangedCallback`and
verification callbacks, either`XmNmodifyVerifyCallback`or`XmNmodifyVerifyCallbackWcs`, or both. If both verification
callback lists are registered, the procedures of the`XmNmodifyVerifyCallback`list are executed first and the resulting
data is passed to the`XmNmodifyVerifyCallbackWcs`callbacks.
If the`XmNcursorPosition`resource is greater than or is the same value as`from_pos`, the`XmNmotionVerifyCallback`is called.

* **`widget`** 

Specifies the TextField widget ID
* **`from_pos`** 

Specifies the start position of the text to be replaced
* **`to_pos`** 

Specifies the end position of the text to be replaced
* **`wcstring`** 

Specifies the wide character string value to be added to the TextField widget


For a complete definition of TextField and its associated resources, see
&cdeman.XmTextField;.
## RELATED


&cdeman.XmTextField; and
&cdeman.XmTextFieldReplace;.