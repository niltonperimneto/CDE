# XmTextInsert
library call`XmTextInsert`A Text function that inserts a character string into a text stringXmTextInsertText functionsXmTextInsert#include <Xm/Text.h>void`XmTextInsert`WidgetwidgetXmTextPositionpositionchar* value
## DESCRIPTION


`XmTextInsert`inserts a character string into the text string in
the Text widget. The character positions begin at 0 (zero) and are numbered
sequentially from the beginning of the text. For example, to insert a
string after the fourth character, the parameter`position`must be
4.

This routine calls the widget's`XmNvalueChangedCallback`and
verification callbacks, either`XmNmodifyVerifyCallback`or`XmNmodifyVerifyCallbackWcs`, or both. If both verification
callback lists are registered, the procedures of the`XmNmodifyVerifyCallback`list are executed first and the resulting
data is passed to the`XmNmodifyVerifyCallbackWcs`callbacks.
If the`XmNcursorPosition`resource is greater than or is the same value as`position`, the`XmNmotionVerifyCallback`is called.

Note that, if`value`is a null string, no callbacks will be
generated, since no modifications will have been made.

* **`widget`** 

Specifies the Text widget ID.
* **`position`** 

Specifies the position in the text string where the character string is
to be inserted.
* **`value`** 

Specifies the character string value to be added to the text widget.


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RELATED


&cdeman.XmText; and
&cdeman.XmTextInsertWcs;.