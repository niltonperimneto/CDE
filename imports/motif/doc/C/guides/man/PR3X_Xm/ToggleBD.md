# XmToggleButtonGadgetSetState
library call`XmToggleButtonGadgetSetState`A ToggleButtonGadget function that sets or changes the current stateXmToggleButtonGadgetSet\\%StateToggleButtonGadget functionsXmToggleButtonGadgetSet\\%State#include <Xm/ToggleBG.h>void`XmToggleButtonGadgetSetState`WidgetwidgetBooleanstateBooleannotify
## DESCRIPTION


`XmToggleButtonGadgetSetState`sets or changes the ToggleButtonGadget's
current state.

* **`widget`** 

Specifies the ToggleButtonGadget widget ID.
* **`state`** 

Specifies a Boolean value that indicates whether the ToggleButtonGadget state is
selected or unselected. If the value is True, the button state is selected;
if it is False, the button state is unselected.
* **`notify`** 

Indicates whether`XmNvalueChangedCallback`is called;
it can be either True or False.
The`XmNvalueChangedCallback`is only called when this function
changes the state of the ToggleButtonGadget.
When this argument is True and the ToggleButtonGadget is a child of a
RowColumn widget whose`XmNradioBehavior`is True, setting the
ToggleButtonGadget causes other ToggleButton and ToggleButtonGadget
children of the RowColumn to be unselected.


For a complete definition of ToggleButtonGadget and
its associated resources, see &cdeman.XmToggleButtonGadget;.
## RELATED


&cdeman.XmToggleButtonGadget;.