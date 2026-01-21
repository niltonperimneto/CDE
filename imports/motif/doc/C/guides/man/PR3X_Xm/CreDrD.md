# XmCreateDropDownComboBox
library call`XmCreateDropDownComboBox`The Drop-down ComboBox widget creation function#include <Xm/ComboBox.h>Widget`XmCreateDropDownComboBox`WidgetparentStringnameArgListarglistCardinalarg_count
## DESCRIPTION


`XmCreateDropDownComboBox`creates an instance of a ComboBox widget of`XmNcomboBoxType``XmDROP_DOWN_COMBO_BOX`and returns
the associated widget ID.

* **`parent`** 

Specifies the parent widget ID.
* **`name`** 

Specifies the name of the created widget.
* **`arglist`** 

Specifies the argument list.
* **`arg_count`** 

Specifies the number of attribute/value pairs in the argument list
(`arglist`).


For a complete definition of ComboBox and its associated resources,
see &cdeman.XmComboBox;.
## RETURN


Returns the ComboBox widget ID.
## RELATED


&cdeman.XmComboBox;.