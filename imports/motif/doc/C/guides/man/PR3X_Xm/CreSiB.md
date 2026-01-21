# XmCreateSimpleMenuBar
library call`XmCreateSimpleMenuBar`A RowColumn widget convenience creation functionXmCreateSimpleMenuBarcreation functionsXmCreateSimpleMenuBar#include <Xm/RowColumn.h>Widget`XmCreateSimpleMenuBar`WidgetparentStringnameArgListarglistCardinalargcount
## DESCRIPTION


`XmCreateSimpleMenuBar`creates an instance of a RowColumn widget of
type`XmMENU_BAR`and returns the associated widget ID.

This routine creates a MenuBar and its CascadeButtonGadget children.
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
The only button type allowed in the`XmNbuttonType`resource is`XmCASCADEBUTTON`.
For a complete definition of RowColumn and its associated resources, see
&cdeman.XmRowColumn;.
## RETURN


Returns the RowColumn widget ID.
## RELATED


&cdeman.XmCreateMenuBar;,
&cdeman.XmCreateRowColumn;,
&cdeman.XmRowColumn;, and
&cdeman.XmVaCreateSimpleMenuBar;.