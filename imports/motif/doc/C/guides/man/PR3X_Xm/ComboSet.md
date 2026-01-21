# XmComboBoxSetItem
library call`XmComboBoxSetItem`set an item in the XmComboBox list#include <Xm/ComboBox.h>void`XmComboBoxSetItem`WidgetwXmStringitem
## DESCRIPTION


The`XmComboBoxSetItem`function selects an item in the XmList of the given XmComboBox
widget and makes it the first visible item in the list.

The`w`argument specifies the XmComboBox widget ID.

The`item`argument specifies theXmStringfor the item to be set in the XmComboBox.
If the`item`is not found on the list,`XmComboBoxSetItem`notifies the user via the`XtWarning`function.
## RETURN VALUE


The`XmComboBoxSetItem`function returns no value.
## SEE ALSO


&cdeman.XmComboBoxAddItem;, &cdeman.XmComboBoxDeletePos;, &cdeman.XmComboBoxSelectItem;;