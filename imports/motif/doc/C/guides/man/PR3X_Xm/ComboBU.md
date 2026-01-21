# XmComboBoxUpdate
library call`XmComboBoxUpdate`A ComboBox function that resynchronizes data#include <Xm/ComboBox.h>void`XmComboBoxUpdate`Widgetwidget
## DESCRIPTION


`XmComboBoxUpdate`resynchronizes the internal data structures of a
specified ComboBox widget. This function is useful when an
application manipulates ComboBox's child widgets, possibly changing
data structures. For example, you might want to use the`XmComboBoxUpdate`function after a ComboBox List child selection
has been changed without notification.

* **`widget`** 

Specifies the ComboBox widget ID.

## RELATED INFORMATION


&cdeman.XmComboBox;.