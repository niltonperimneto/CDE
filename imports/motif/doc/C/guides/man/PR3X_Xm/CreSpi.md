# XmCreateSpinBox
library call`XmCreateSpinBox`The SpinBox creation functionXmCreateSpinBoxcreation functionsXmCreateSpinBox#include <Xm/SpinB.h>
## DESCRIPTION


`XmCreateSpinBox`creates a SpinBox widget.

This function creates a SpinBox with two arrows,
but without any traversable children (choices to spin).
The application can create text children to go with this parent SpinBox
using`XmCreateTextField`or`XmCreateText`.

* **`parent`** 

Specifies the parent widget ID
* **`name`** 

Specifies the name of the created widget
* **`arglist`** 

Specifies the argument list
* **`argcount`** 

Specifies the number of attribute/value pairs in the argument list
(`arglist`)


For a complete definition of SpinBox and its associated resources, see
&cdeman.XmSpinBox;.
## RETURN


Returns the SpinBox widget ID.
## RELATED


&cdeman.XmSpinBox;