# XmTextReplaceWcs
library call`XmTextReplaceWcs`A Text function that replaces part of
a wide character string in a Text widgetXmTextReplaceWcsText functionsXmTextReplaceWcs#include <Xm/Text.h>void`XmTextReplaceWcs`WidgetwidgetXmTextPositionfrom_posXmTextPositionto_poswchar_t *wcstring
## DESCRIPTION


`XmTextReplaceWcs`replaces part of the wide character
string in the Text widget. The character positions begin at
zero and are numbered sequentially from the beginning of the text.

An example text replacement would be to replace
the second and third characters in the text string. To accomplish this,
the`from_pos`parameter must be 1 and the`to_pos`parameter must be 3. To
insert a string after the fourth character, both the`from_pos`and`to_pos`parameters must be 4.

This routine calls the widget's`XmNvalueChangedCallback`and
verification callbacks, either`XmNmodifyVerifyCallback`or`XmNmodifyVerifyCallbackWcs`, or both. If both verification
callback lists are registered, the procedures of the`XmNmodifyVerifyCallback`list are executed first and the resulting
data is passed to the`XmNmodifyVerifyCallbackWcs`callbacks.
This routine calls the widget's`XmNmotionVerifyCallback`callback
when`from_pos`is less than or equal to the cursor position.

* **`widget`** 

Specifies the Text widget ID
* **`from_pos`** 

Specifies the start position of the text to be replaced
* **`to_pos`** 

Specifies the end position of the text to be replaced
* **`wcstring`** 

Specifies the wide character string value to be added to the Text widget


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RELATED


&cdeman.XmText; and
&cdeman.XmTextReplace;.