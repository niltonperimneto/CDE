# XmCreateSimpleRadioBox
library call`XmCreateSimpleRadioBox`A RowColumn widget convenience creation functionXmCreateSimpleRadioBoxcreation functionsXmCreateSimpleRadioBox#include <Xm/RowColumn.h>Widget`XmCreateSimpleRadioBox`WidgetparentStringnameArgListarglistCardinalargcount
## DESCRIPTION


`XmCreateSimpleRadioBox`creates an instance of a RowColumn widget
of type`XmWORK_AREA`and returns the associated widget ID.

This routine creates a RadioBox and its ToggleButtonGadget children.
The name of each button is`button_``n`, where`n`is an integer
from 0 (zero) to the number of buttons in the menu minus 1.
Buttons are named and created in the order they are specified
in the RowColumn simple menu creation resources supplied in the argument
list.

* **`parent`** 

Specifies the parent widget ID
* **`name`** 

Specifies the name of the created widget
* **`arglist`** 

Specifies the argument list
* **`argcount`** 

Specifies the number of attribute/value pairs in the argument list
(`arglist`)


A number of resources exist specifically for use with this and
other simple menu creation routines.
The only button type allowed in the`XmNbuttonType`resource is`XmRADIOBUTTON`.
For a complete definition of RowColumn and its associated resources, see
&cdeman.XmRowColumn;.
## RETURN


Returns the RowColumn widget ID.
## RELATED


&cdeman.XmCreateRadioBox;,
&cdeman.XmCreateRowColumn;,
&cdeman.XmCreateSimpleCheckBox;,
&cdeman.XmRowColumn;, and
&cdeman.XmVaCreateSimpleRadioBox;.