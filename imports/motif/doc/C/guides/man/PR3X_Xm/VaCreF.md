# XmVaCreateSimpleRadioBox
library call`XmVaCreateSimpleRadioBox`A RowColumn widget convenience creation functionXmVaCreateSimpleRadioBoxcreation functionsXmVaCreateSimpleRadioBox#include <Xm/RowColumn.h>Widget`XmVaCreateSimpleRadioBox`WidgetparentStringnameintbutton_setXtCallbackProccallback
## DESCRIPTION


`XmVaCreateSimpleRadioBox`creates an instance of a RowColumn widget
of type`XmWORK_AREA`and returns the associated widget ID.
This routine uses the ANSI C variable-length argument list (`varargs`)
calling convention.

This routine creates a RadioBox and its ToggleButtonGadget children.
The name of each button is`button_``n`, where`n`is an integer
from 0 (zero) to the number of buttons in the menu minus 1.

* **`parent`** 

Specifies the parent widget ID.
* **`name`** 

Specifies the name of the created widget.
* **`button_set`** 

Specifies which button is initially set.
The value is the integer`n`in the button name`button_``n`.
* **`callback`** 

Specifies a callback procedure to be called when a button's value
changes.
This callback function is added to each button after creation as the
button's`XmNvalueChangedCallback`.
The callback function is called when a button's value changes, and the
button number is returned in the`client_data`field.


The variable portion of the argument list consists of groups of
arguments.
The first argument in each group is a constant or a string and
determines which arguments follow in that group.
The last argument in the list must be NULL.
Following are the possible first arguments in each group of`varargs`:

* **`XmVaRADIOBUTTON`** 

This is followed by four additional arguments. The set specifies one
button in the RadioBox and some of its resource values. Following are
the additional four arguments, in order:

* **`label`** 

The label string, of typeXmString.
* **`mnemonic`** 

The mnemonic, of type`KeySym`. This is ignored in this release.
* **`accelerator`** 

The accelerator, of typeString. This is ignored in this release.
* **`accelerator_text`** 

The accelerator text, of typeXmString. This is ignored in this
release.

* **`resource_name`** 

This is followed by one additional argument, the value of the resource,
of type`XtArgVal`. The pair specifies a resource and its value for the
RowColumn widget.
* **`XtVaTypedArg`** 

This is followed by four additional arguments. The set specifies a
resource and its value for the RowColumn widget. A resource type
conversion is performed if necessary. Following are the additional four
arguments, in this order:

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


&cdeman.XmCreateRadioBox;,
&cdeman.XmCreateRowColumn;,
&cdeman.XmCreateSimpleCheckBox;,
&cdeman.XmCreateSimpleRadioBox;,
&cdeman.XmRowColumn;, and
&cdeman.XmVaCreateSimpleCheckBox;,