# XmTextSetAddMode
library call`XmTextSetAddMode`A Text function that sets the state of Add modeXmTextSetAddModeText functionsXmTextSetAddMode#include <Xm/Text.h>void`XmTextSetAddMode`WidgetwidgetBooleanstate
## DESCRIPTION


`XmTextSetAddMode`controls whether or not the Text widget is in Add
mode. When the widget is in Add mode, the insert cursor can be moved
without disturbing the primary selection.

* **`widget`** 

Specifies the Text widget ID
* **`state`** 

Specifies whether or not the widget is in Add mode. A value of True
turns on Add mode; a value of False turns off Add mode.


For a complete definition of Text and its associated resources, see
&cdeman.XmText;.
## RELATED


&cdeman.XmText;.