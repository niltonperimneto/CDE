# XmToggleButtonSetValue
library call`XmToggleButtonSetValue`A ToggleButton function that sets or changes the current stateXmToggleButtonSetValueToggleButton functionsXmToggleButtonSetValue#include <Xm/ToggleB.h>void`XmToggleButtonSetValue`WidgetwidgetXmToggleButtonStatestateBooleannotify
## DESCRIPTION


`XmToggleButtonSetValue`sets or changes the ToggleButton's current state.

* **`widget`** 

Specifies the ToggleButton widget ID.
* **`state`** 

Specifies whether the ToggleButton state is
selected or unselected. If the value is True, the button state is selected;
if it is False, the button state is unselected, if it is`XmINDETERMINATE`, the button state is neither.
* **`notify`** 

Indicates whether`XmNvalueChangedCallback`is called;
it can be either True or False.
The`XmNvalueChangedCallback`is only called when this function
changes the state of the ToggleButton.
When this argument is True and the ToggleButton is a child of a
RowColumn widget whose`XmNradioBehavior`is True, setting the
ToggleButton causes other ToggleButton and ToggleButtonGadget
children of the RowColumn to be unselected.


For a complete definition of ToggleButton and
its associated resources, see &cdeman.XmToggleButton;.
## RELATED


&cdeman.XmToggleButton;.