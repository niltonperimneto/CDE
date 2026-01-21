# XmComboBoxAddItem
library call`XmComboBoxAddItem`add an item to the ComboBox widget#include <Xm/ComboBox.h>void`XmComboBoxAddItem`WidgetwXmStringitemintposBooleanunique
## DESCRIPTION


The`XmComboBoxAddItem`function adds the given item to the XmComboBox at the given position.

The`w`argument specifies the XmComboBox widget ID.

The`item`argument specifies theXmStringfor the new item.

Theposargument specifies the position of the new item.

Theuniqueargument specifies if this item should duplicate an identical item or not.
## RETURN VALUE


The`XmComboBoxAddItem`function returns no value.
## APPLICATION USAGE


The functions`XmComboBoxAddItem`and`XmComboBoxDeletePos`have different naming conventions (Item versus Pos)
because of the objects they are manipulating.
The Item is a string to be added,
the Pos is a numeric position number.
## SEE ALSO


&cdeman.XmComboBoxDeletePos;, &cdeman.XmComboBoxSetItem;, &cdeman.XmComboBoxSelectItem;.