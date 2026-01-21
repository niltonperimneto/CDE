# XmImUnregister
library call`XmImUnregister`An input manager function that removes a widget
from association with its input managerXmImUnregisterinput manager functionsXmImUnregister#include <Xm/XmIm.h>void`XmImUnregister`Widgetwidget
## DESCRIPTION


`XmImUnregister`removes the specified widget from the list of
widgets registered for input by the input manager.

Note that the Text, TextField, and List widgets already call the`XmImRegister`internally. You should call the`XmImUnregister`function for
these widgets before calling`XmImRegister`.

* **`widget`** 

Specifies the ID of the widget to be unregistered

## RELATED


&cdeman.XmImRegister;.