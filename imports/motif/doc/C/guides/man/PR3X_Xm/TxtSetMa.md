# XmTextSetMaxLength
library call`XmTextSetMaxLength`A Text function that sets the value of the current maximum allowable length of a text string entered from the keyboardXmTextSetMaxLengthText functionsXmTextSetMaxLength#include <Xm/Text.h>void`XmTextSetMaxLength`Widgetwidgetintmax_length
## DESCRIPTION


`XmTextSetMaxLength`sets the value of the current maximum allowable
length of the text string in the Text widget. The maximum allowable length
prevents the user from entering a text string from the keyboard
that is larger than this limit.
Strings that are entered using the`XmNvalue`(or`XmNvalueWcs`)
resource, or the`XmTextSetString`(or`XmTextSetStringWcs`)
function ignore this resource.

* **`widget`** 

Specifies the Text widget ID
* **`max_length`** 

Specifies the maximum allowable length of the text string


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RELATED


&cdeman.XmText;,
&cdeman.XmTextSetString;, and
&cdeman.XmTextSetStringWcs;.