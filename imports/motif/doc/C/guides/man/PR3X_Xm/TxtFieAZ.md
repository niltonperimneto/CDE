# XmTextFieldSetAddMode
library call`XmTextFieldSetAddMode`A TextField function that sets the state of Add modeXmTextFieldSetAddModeTextField functionsXmTextFieldSetAddMode#include <Xm/TextF.h>void`XmTextFieldSetAddMode`WidgetwidgetBooleanstate
## DESCRIPTION


`XmTextFieldSetAddMode`controls whether or not the TextField widget is in Add
mode. When the widget is in Add mode, the insert cursor can be moved
without disturbing the primary selection.

* **`widget`** 

Specifies the TextField widget ID
* **`state`** 

Specifies whether or not the widget is in Add mode. A value of True
turns on Add mode; a value of False turns off Add mode.


For a complete definition of TextField and its associated resources, see
&cdeman.XmTextField;.
## RELATED


&cdeman.XmTextField;.