# XmCreateRadioBox
library call`XmCreateRadioBox`A RowColumn widget convenience creation functionXmCreateRadioBoxcreation functionsXmCreateRadioBox#include <Xm/RowColumn.h>Widget`XmCreateRadioBox`WidgetparentStringnameArgListarglistCardinalargcount
## DESCRIPTION


`XmCreateRadioBox`creates an instance of a RowColumn widget
of type`XmWORK_AREA`and returns the
associated widget ID. Typically,
this is a composite widget that contains multiple
ToggleButtonGadgets.
The RadioBox arbitrates and ensures that at most one
ToggleButtonGadget is on at any time.

Unless the application supplies other values in the`arglist`, this
function provides initial values for several RowColumn resources.
It initializes`XmNpacking`to`XmPACK_COLUMN`,`XmNradioBehavior`to True,`XmNisHomogeneous`to True, and`XmNentryClass`toXmToggleButtonGadgetClass.

In a RadioBox, the ToggleButton or ToggleButtonGadget resource`XmNindicatorType`defaults to`XmONE_OF_MANY`, and the
ToggleButton or ToggleButtonGadget resource`XmNvisibleWhenOff`defaults to True.

This routine is provided as a convenience function for creating
RowColumn widgets.

* **`parent`** 

Specifies the parent widget ID
* **`name`** 

Specifies the name of the created widget
* **`arglist`** 

Specifies the argument list
* **`argcount`** 

Specifies the number of attribute/value pairs in the argument list
(`arglist`)


For a complete definition of RowColumn and its associated resources, see
&cdeman.XmRowColumn;.
## RETURN


Returns the RowColumn widget ID.
## RELATED


&cdeman.XmCreateRowColumn;,
&cdeman.XmCreateSimpleCheckBox;,
&cdeman.XmCreateSimpleRadioBox;,
&cdeman.XmCreateWorkArea;,
&cdeman.XmRowColumn;,
&cdeman.XmVaCreateSimpleCheckBox;, and
&cdeman.XmVaCreateSimpleRadioBox;.