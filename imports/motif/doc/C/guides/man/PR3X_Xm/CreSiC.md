# XmCreateSimpleOptionMenu
library call`XmCreateSimpleOptionMenu`A RowColumn widget convenience creation functionXmCreateSimpleOptionMenucreation functionsXmCreateSimpleOptionMenu#include <Xm/RowColumn.h>Widget`XmCreateSimpleOptionMenu`WidgetparentStringnameArgListarglistCardinalargcount
## DESCRIPTION


`XmCreateSimpleOptionMenu`creates an instance of a RowColumn widget
of type`XmMENU_OPTION`and returns the associated widget ID.

This routine creates an OptionMenu and its submenu containing
PushButtonGadget or CascadeButtonGadget children.
The name of each button is`button_``n`, where`n`is an integer
from 0 (zero) to the number of buttons in the menu minus 1.
The name of each separator is`separator_``n`, where`n`is an integer
from 0 (zero) to the number of separators in the menu minus 1.
Buttons and separators are named and created in the order they
are specified in the RowColumn simple menu creation resources supplied
in the argument list.

* **`parent`** 

Specifies the parent widget ID
* **`name`** 

Specifies the name of the created widget
* **`arglist`** 

Specifies the argument list
* **`argcount`** 

Specifies the number of attribute/value pairs in the argument list
(`arglist`)


The user can specify resources in a resource file for the automatically
created widgets and gadgets of an OptionMenu.
These widgets (or gadgets) and the associated
OptionMenu areas are

* **Option Menu Label Gadget** 

`OptionLabel`
* **Option Menu Cascade Button** 

`OptionButton`


A number of resources exist specifically for use with this and
other simple menu creation routines.
The only button types allowed in the`XmNbuttonType`resource are`XmPUSHBUTTON`,`XmCASCADEBUTTON`,`XmSEPARATOR`, and`XmDOUBLE_SEPARATOR`.
For a complete definition of RowColumn and its associated resources, see
&cdeman.XmRowColumn;.
## RETURN


Returns the RowColumn widget ID.
## RELATED


&cdeman.XmCreateOptionMenu;,
&cdeman.XmCreateRowColumn;,
&cdeman.XmRowColumn;, and
&cdeman.XmVaCreateSimpleOptionMenu;.