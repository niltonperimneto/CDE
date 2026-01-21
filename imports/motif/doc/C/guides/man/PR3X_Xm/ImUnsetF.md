# XmImUnsetFocus
library call`XmImUnsetFocus`An input manager function that notifies an input
method that a widget has lost input focusXmImUnsetFocusinput manager functionsXmImUnsetFocus#include <Xm/XmIm.h>void`XmImUnsetFocus`Widgetwidget
## DESCRIPTION


`XmImUnsetFocus`unsets a specified widget's focus, then
notifies the input manager that the specified widget has lost its input focus.

Note that the Text, TextField, and List widgets already call the`XmImUnsetFocus`internally. Therefore, further calls to the`XmImUnsetFocus`function for those widgets are unnecessary.

* **`widget`** 

Specifies the ID of the widget registered with the input manager

## RELATED


&cdeman.XmImSetFocusValues; and &cdeman.XmImVaSetFocusValues;.