# XmTextFieldGetMaxLength
library call`XmTextFieldGetMaxLength`A TextField function that accesses the value of the current maximum allowable length of a text string entered from the keyboardXmTextFieldGetMaxLengthTextField functionsXmTextFieldGetMaxLength#include <Xm/TextF.h>int`XmTextFieldGetMaxLength`Widgetwidget
## DESCRIPTION


`XmTextFieldGetMaxLength`accesses the value of the current maximum allowable
length of the text string in the TextField widget entered from the
keyboard. The maximum allowable length
prevents the user from entering a text string larger than this limit.
Note that the maximum allowable length is the same as the value
of the widget's`XmNmaxLength`resource.

* **`widget`** 

Specifies the TextField widget ID


For a complete definition of TextField and its associated resources, see
&cdeman.XmTextField;.
## RETURN


Returns the integer value that indicates the string's maximum allowable
length that can be entered from the keyboard.
## RELATED


&cdeman.XmTextField;.