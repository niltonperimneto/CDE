# XmListSetAddMode
library call`XmListSetAddMode`A List function that sets add mode in the listXmListSetAddModeList functionsXmListSetAddMode#include <Xm/List.h>void`XmListSetAddMode`WidgetwidgetBooleanstate
## DESCRIPTION


`XmListSetAddMode`allows applications
control over Add Mode in the extended selection model.
This function ensures that the mode it sets is compatible with the
selection policy (`XmNselectionPolicy`) of the widget. For
example, it cannot put the widget into add mode when the value of`XmNselectionPolicy`is`XmBROWSE_SELECT`.

* **`widget`** 

Specifies the ID of the List widget
* **`state`** 

Specifies whether to activate or deactivate Add Mode.
If`state`is True, Add Mode is activated. If`state`is
False, Add Mode is deactivated.


For a complete definition of List and its associated resources, see
&cdeman.XmList;.
## RELATED


&cdeman.XmList;.