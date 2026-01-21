# XmVaCreateSimplePopupMenu
library call`XmVaCreateSimplePopupMenu`A RowColumn widget convenience creation functionXmVaCreateSimplePopup\\%Menucreation functionsXmVaCreateSimplePopup\\%Menu#include <Xm/RowColumn.h>Widget`XmVaCreateSimplePopupMenu`WidgetparentStringnameXtCallbackProccallback
## DESCRIPTION


`XmVaCreateSimplePopupMenu`creates an instance of a RowColumn widget
of type`XmMENU_POPUP`and returns the associated widget ID.
This routine uses the ANSI C variable-length argument list (`varargs`)
calling convention.

This routine creates a Popup menu pane and its button children.
The name of each button is`button_``n`, where`n`is an integer
from 0 (zero) to the number of buttons in the menu minus 1.
The name of each separator is`separator_``n`, where`n`is an
integer from 0 (zero) to the number of separators in the menu minus 1.
The name of each title is`label_``n`, where`n`is an
integer from 0 (zero) to the number of titles in the menu minus 1.
Buttons, separators, and titles are named and created in the order in
which they are specified in the variable portion of the argument list.

* **`parent`** 

Specifies the widget ID of the parent of the MenuShell
* **`name`** 

Specifies the name of the created widget
* **`callback`** 

Specifies a callback procedure to be called when a button is activated
or when its value changes.
This callback function is added to each button after creation.
For a CascadeButtonGadget or a PushButtonGadget, the callback is added
as the button's`XmNactivateCallback`, and it is called when the
button is activated.
For a ToggleButtonGadget, the callback is added as the button's`XmNvalueChangedCallback`, and it is called when the button's value
changes.
The button number is returned in the`client_data`field.


The variable portion of the argument list consists of groups of
arguments.
The first argument in each group is a constant or a string and
determines which arguments follow in that group.
The last argument in the list must be NULL.
The following list describes the possible first arguments in
each group of`varargs`.

* **`XmVaCASCADEBUTTON`** 

This is followed by two additional arguments. The set specifies one
button in the PopupMenu and some of its resource
values. The button created is a CascadeButtonGadget. Following are the
additional two arguments, in order:

* **`label`** 

The label string, of typeXmString
* **`mnemonic`** 

The mnemonic, of type`KeySym`

* **`XmVaPUSHBUTTON`** 

This is followed by four additional arguments. The set specifies one
button in the PopupMenu and some of its resource
values. The button created is a PushButtonGadget. Following are the
additional four arguments, in order:

* **`label`** 

The label string, of typeXmString
* **`mnemonic`** 

The mnemonic, of type`KeySym`
* **`accelerator`** 

The accelerator, of typeString
* **`accelerator_text`** 

The accelerator text, of typeXmString

* **`XmVaRADIOBUTTON`** 

This is followed by four additional arguments. The set specifies one
button in the PopupMenu and some of its resource
values. The button created is a ToggleButtonGadget. Following are the
additional four arguments, in order:

* **`label`** 

The label string, of typeXmString
* **`mnemonic`** 

The mnemonic, of type`KeySym`
* **`accelerator`** 

The accelerator, of typeString
* **`accelerator_text`** 

The accelerator text, of typeXmString

* **`XmVaCHECKBUTTON`** 

This is followed by four additional arguments. The set specifies one
button in the PopupMenu and some of its resource
values. The button created is a ToggleButtonGadget. Following are the
additional four arguments, in order:

* **`label`** 

The label string, of typeXmString
* **`mnemonic`** 

The mnemonic, of type`KeySym`
* **`accelerator`** 

The accelerator, of typeString
* **`accelerator_text`** 

The accelerator text, of typeXmString

* **`XmVaTITLE`** 

This is followed by one additional argument. The pair specifies a
title LabelGadget in the PopupMenu. Following is the
additional argument:

* **`title`** 

The title string, of typeXmString

* **`XmVaSEPARATOR`** 

This is followed by no additional arguments. It specifies one separator
in the PopupMenu.
* **`XmVaDOUBLE_SEPARATOR`** 

This is followed by no additional arguments. It specifies one separator
in the PopupMenu. The separator type is`XmDOUBLE_LINE`.
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


&cdeman.XmCreatePopupMenu;,
&cdeman.XmCreateRowColumn;,
&cdeman.XmCreateSimplePopupMenu;, and
&cdeman.XmRowColumn;.