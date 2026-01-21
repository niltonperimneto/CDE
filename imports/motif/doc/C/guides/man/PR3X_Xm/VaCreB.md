# XmVaCreateSimpleMenuBar
library call`XmVaCreateSimpleMenuBar`A RowColumn widget convenience creation functionXmVaCreateSimpleMenuBarcreation functionsXmVaCreateSimpleMenuBar#include <Xm/RowColumn.h>Widget`XmVaCreateSimpleMenuBar`WidgetparentStringname
## DESCRIPTION


`XmVaCreateSimpleMenuBar`creates an instance of a RowColumn widget of
type`XmMENU_BAR`and returns the associated widget ID.
This routine uses the ANSI C variable-length argument list (`varargs`)
calling convention.

This routine creates a MenuBar and its CascadeButtonGadget children.
The name of each button is`button_``n`, where`n`is an integer
from 0 (zero) to the number of buttons in the menu minus 1.
Buttons are named and created in the order in which they are specified
in the variable portion of the argument list.

* **`parent`** 

Specifies the parent widget ID
* **`name`** 

Specifies the name of the created widget


The variable portion of the argument list consists of groups of
arguments.
The first argument in each group is a constant or a string and
determines which arguments follow in that group.
The last argument in the list must be NULL.
Following are the possible first arguments in each group of`varargs`:

* **`XmVaCASCADEBUTTON`** 

This is followed by two additional arguments. The set specifies one
button in the MenuBar and some of its resource values. Following are
the additional two arguments, in order:

* **`label`** 

The label string, of typeXmString
* **`mnemonic`** 

The mnemonic, of type`KeySym`

* **`resource_name`** 

This is followed by one additional argument, the value of the resource,
of type`XtArgVal`. The pair specifies a resource and its value for the
RowColumn widget.
* **`XtVaTypedArg`** 

This is followed by four additional arguments. The set specifies a
resource and its value for the RowColumn widget. A resource type
conversion is performed if necessary. Following are the additional four
arguments, in order:

* **`name`** 

The resource name, of typeString
* **`type`** 

The type of the resource value supplied, of typeString
* **`value`** 

The resource value (or a pointer to the resource value, depending on the
type and size of the value), of type`XtArgVal`
* **`size`** 

The size of the resource value in bytes, of type`int`

* **`XtVaNestedList`** 

This is followed by one additional argument of type`XtVarArgsList`. This
argument is a nested list of`varargs`returned by`XtVaCreateArgsList`.


For more information on variable-length argument lists, see the X
Toolkit Intrinsics documentation.

A number of resources exist specifically for use with this and
other simple menu creation routines.
For a complete definition of RowColumn and its associated resources, see
&cdeman.XmRowColumn;.
## RETURN


Returns the RowColumn widget ID.
## RELATED


&cdeman.XmCreateMenuBar;,
&cdeman.XmCreateRowColumn;,
&cdeman.XmCreateSimpleMenuBar;, and
&cdeman.XmRowColumn;.