# XmSimpleSpinBoxSetItem
library call`XmSimpleSpinBoxSetItem`set an item in the XmSimpleSpinBox list#include <Xm/SpinB.h>void`XmSimpleSpinBoxSetItem`WidgetwXmStringitem
## DESCRIPTION


The`XmSimpleSpinBoxSetItem`function selects an item in the list of the given XmSimpleSpinBox
widget and makes it the current value.

The`w`argument specifies the widget ID.

The`item`argument specifies theXmStringfor the item to be set in the XmSimpleSpinBox.
If the`item`is not found on the list,`XmSimpleSpinBoxSetItem`notifies the user via the`XtWarning`function.
## RETURN VALUE


The`XmSimpleSpinBoxSetItem`function returns no value.
## SEE ALSO
