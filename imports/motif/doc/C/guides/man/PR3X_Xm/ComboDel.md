# XmComboBoxDeletePos
library call`XmComboBoxDeletePos`Delete a XmComboBox item#include <Xm/ComboBox.h>void`XmComboBoxDeletePos`Widgetwintpos
## DESCRIPTION


The`XmComboBoxDeletePos`function deletes a specified item from a XmComboBox widget.

The`w`argument specifies the XmComboBox widget ID.

Theposargument specifies the position of the item to be deleted.
## RETURN VALUE


The`XmComboBoxDeletePos`function returns no value.
## APPLICATION USAGE


The functions`XmComboBoxAddItem`and`XmComboBoxDeletePos`have different naming conventions (Item versus Pos)
because of the objects they are manipulating.
The Item is a string to be added,
the Pos is a numeric position number.
## SEE ALSO


&cdeman.XmComboBoxAddItem;, &cdeman.XmComboBoxSetItem;, &cdeman.XmComboBoxSelectItem;.