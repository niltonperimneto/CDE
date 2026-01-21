# XmComboBoxSelectItem
library call`XmComboBoxSelectItem`select a XmComboBox item#include <Xm/ComboBox.h>void`XmComboBoxSelectItem`WidgetwXmStringitem
## DESCRIPTION


The`XmComboBoxSelectItem`function selects an item in the XmList of the XmComboBox
widget.

The`w`argument specifies the XmComboBox widget ID.

The`item`argument specifies theXmStringof the item to be selected.
If the`item`is not found on the list,`XmComboBoxSelectItem`notifies the user via the`XtWarning`function.
## RETURN VALUE


The`XmComboBoxSelectItem`function returns no value.
## SEE ALSO


&cdeman.XmComboBoxAddItem;, &cdeman.XmComboBoxDeletePos;, &cdeman.XmComboBoxSetItem;;