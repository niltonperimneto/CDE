# XmCreateSimplePopupMenu
library call`XmCreateSimplePopupMenu`A RowColumn widget convenience creation functionXmCreateSimplePopupMenucreation functionsXmCreateSimplePopupMenu#include <Xm/RowColumn.h>Widget`XmCreateSimplePopupMenu`WidgetparentStringnameArgListarglistCardinalargcount
## DESCRIPTION


`XmCreateSimplePopupMenu`creates an instance of a RowColumn widget
of type`XmMENU_POPUP`and returns the associated widget ID.

This routine creates a Popup menu pane and its button children.
The name of each button is`button_``n`, where`n`is an integer
from 0 (zero) to the number of buttons in the menu minus 1.
The name of each separator is`separator_``n`, where`n`is an integer
from 0 (zero) to the number of separators in the menu minus 1.
The name of each title is`label_``n`, where`n`is an integer
from 0 (zero) to the number of titles in the menu minus 1.
Buttons, separators, and titles are named and created in the order in
which they are specified in the RowColumn simple menu creation resources
supplied in the argument list.

* **`parent`** 

Specifies the widget ID of the parent of the MenuShell
* **`name`** 

Specifies the name of the created widget
* **`arglist`** 

Specifies the argument list
* **`argcount`** 

Specifies the number of attribute/value pairs in the argument list
(`arglist`)


A number of resources exist specifically for use with this and
other simple menu creation routines.
The only button types allowed in the`XmNbuttonType`resource are`XmCASCADEBUTTON`,`XmPUSHBUTTON`,`XmRADIOBUTTON`,`XmCHECKBUTTON`,`XmTITLE`,`XmSEPARATOR`, and`XmDOUBLE_SEPARATOR`.
For a complete definition of RowColumn and its associated resources, see
&cdeman.XmRowColumn;.
## RETURN


Returns the RowColumn widget ID.
## RELATED


&cdeman.XmCreatePopupMenu;,
&cdeman.XmCreateRowColumn;,
&cdeman.XmRowColumn;, and
&cdeman.XmVaCreateSimplePopupMenu;.