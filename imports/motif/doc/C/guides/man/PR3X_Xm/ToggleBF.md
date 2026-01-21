# XmToggleButtonSetState
library call`XmToggleButtonSetState`A ToggleButton function that sets or changes the current stateXmToggleButtonSetStateToggleButton functionsXmToggleButtonSetState#include <Xm/ToggleB.h>void`XmToggleButtonSetState`WidgetwidgetBooleanstateBooleannotify
## DESCRIPTION


`XmToggleButtonSetState`sets or changes the ToggleButton's current state.

* **`widget`** 

Specifies the ToggleButton widget ID.
* **`state`** 

Specifies a Boolean value that indicates whether the ToggleButton state is
selected or unselected. If the value is True, the button state is selected;
if it is False, the button state is unselected.
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