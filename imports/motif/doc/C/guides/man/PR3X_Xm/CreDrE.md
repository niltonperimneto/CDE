# XmCreateDropDownList
library call`XmCreateDropDownList`The Drop-down list ComboBox widget creation function#include <Xm/ComboBox.h>Widget`XmCreateDropDownList`WidgetparentStringnameArgListarglistCardinalarg_count
## DESCRIPTION


`XmCreateDropDownList`creates an instance of a ComboBox widget of`XmNcomboBoxType``XmDROP_DOWN_LIST`and returns
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