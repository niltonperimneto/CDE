# XmTextReplace
library call`XmTextReplace`A Text function that replaces part of a text stringXmTextReplaceText functionsXmTextReplace#include <Xm/Text.h>void`XmTextReplace`WidgetwidgetXmTextPositionfrom_posXmTextPositionto_poschar* value
## DESCRIPTION


`XmTextReplace`replaces part of the text string in the Text widget. The
character positions begin at 0 (zero) and are numbered sequentially from the
beginning of the text.

An example text replacement would be to replace
the second and third characters in the text string. To accomplish this,
the parameter`from_pos`must be 1 and`to_pos`must be 3. To
insert a string after the fourth character, both parameters,`from_pos`and`to_pos`, must be 4.

This routine calls the widget's`XmNvalueChangedCallback`and
verification callbacks, either`XmNmodifyVerifyCallback`or`XmNmodifyVerifyCallbackWcs`, or both. If both verification
callback lists are registered, the procedures of the`XmNmodifyVerifyCallback`list are executed first and the resulting
data is passed to the`XmNmodifyVerifyCallbackWcs`callbacks.
The`XmNmotionVerifyCallback`is generated if`to_pos`is less
than or equal to
the cursor position and the length of`value`is not the same as the
length of the text being replaced, or if the cursor position is between`from_pos`and`to_pos`, and the distance from the cursor
position to`from_pos`is greater than the length of`value`.

* **`widget`** 

Specifies the Text widget ID
* **`from_pos`** 

Specifies the start position of the text to be replaced
* **`to_pos`** 

Specifies the end position of the text to be replaced
* **`value`** 

Specifies the character string value to be added to the text widget


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RELATED


&cdeman.XmText; and
&cdeman.XmTextReplaceWcs;.