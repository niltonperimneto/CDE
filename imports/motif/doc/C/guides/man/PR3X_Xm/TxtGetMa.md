# XmTextGetMaxLength
library call`XmTextGetMaxLength`A Text function that accesses the value of the current maximum allowable length of a text string entered from the keyboardXmTextGetMaxLengthText functionsXmTextGetMaxLength#include <Xm/Text.h>int`XmTextGetMaxLength`Widgetwidget
## DESCRIPTION


`XmTextGetMaxLength`accesses the value of the current maximum allowable
length of the text string in the Text widget entered from the
keyboard. The maximum allowable length
prevents the user from entering a text string larger than this limit.
Note that the maximum allowable length is the same as the value
of the widget's`XmNmaxLength`resource.

* **`widget`** 

Specifies the Text widget ID


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RETURN


Returns the integer value that indicates the string's maximum allowable
length that can be entered from the keyboard.
## RELATED


&cdeman.XmText;.