# XmTextFieldSetMaxLength
library call`XmTextFieldSetMaxLength`A TextField function that sets the value of the current maximum allowable length of a text string entered from the keyboardXmTextFieldSetMaxLengthTextField functionsXmTextFieldSetMaxLength#include <Xm/TextF.h>void`XmTextFieldSetMaxLength`Widgetwidgetintmax_length
## DESCRIPTION


`XmTextFieldSetMaxLength`sets the value of the current maximum allowable
length of the text string in the TextField widget. The maximum allowable length
prevents the user from entering a text string from the keyboard
that is larger than this limit.
Strings that are entered using the`XmNvalue`(or`XmNvalueWcs`) resource, or the`XmTextFieldSetString`(or`XmTextFieldSetStringWcs`)
function ignore this resource.

* **`widget`** 

Specifies the TextField widget ID
* **`max_length`** 

Specifies the maximum allowable length of the text string


For a complete definition of TextField and its associated resources, see
&cdeman.XmTextField;.
## RELATED


&cdeman.XmText;,
&cdeman.XmTextFieldSetString;, and
&cdeman.XmTextFieldSetStringWcs;.