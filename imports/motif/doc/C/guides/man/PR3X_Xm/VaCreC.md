# XmVaCreateSimpleOptionMenu
library call`XmVaCreateSimpleOptionMenu`A RowColumn widget convenience creation functionXmVaCreateSimpleOption\\%Menucreation functionsXmVaCreateSimpleOption\\%Menu#include <Xm/RowColumn.h>Widget`XmVaCreateSimpleOptionMenu`WidgetparentStringnameXmStringoption_labelKeySymoption_mnemonicintbutton_setXtCallbackProccallback
## DESCRIPTION


`XmVaCreateSimpleOptionMenu`creates an instance of a RowColumn widget
of type`XmMENU_OPTION`and returns the associated widget ID.
This routine uses the ANSI C variable-length argument list (`varargs`)
calling convention.

This routine creates an OptionMenu and its Pulldown submenu containing
PushButtonGadget or CascadeButtonGadget children.
The name of each button is`button_``n`, where`n`is an integer
from 0 (zero) to the number of buttons in the menu minus 1.
The name of each separator is`separator_``n`, where`n`is an
integer from 0 (zero) to the number of separators in the menu minus 1.
Buttons and separators are named and created in the order in which they
are specified in the variable portion of the argument list.

* **`parent`** 

Specifies the parent widget ID
* **`name`** 

Specifies the name of the created widget
* **`option_label`** 

Specifies the label string to be used on the left side of the
OptionMenu.
* **`option_mnemonic`** 

Specifies a keysym for a key that, when pressed by the user, posts
the associated Pulldown menu pane.
* **`button_set`** 

Specifies which PushButtonGadget is initially set.
The value is the integer`n`that corresponds to the`n`th
PushButtonGadget specified in the variable portion of the argument list.
Only a PushButtonGadget can be set, and only PushButtonGadgets are
counted in determining the integer`n`.
The first PushButtonGadget is number 0 (zero).
* **`callback`** 

Specifies a callback procedure to be called when a button is activated.
This callback function is added to each button after creation as the
button's`XmNactivateCallback`.
The callback function is called when a button is activated, and the
button number is returned in the`client_data`field.


The variable portion of the argument list consists of groups of
arguments.
The first argument in each group is a constant or a string and
determines which arguments follow in that group.
The last argument in the list must be NULL.
Following are the possible first arguments in each group of`varargs`:

* **`XmVaPUSHBUTTON`** 

This is followed by four additional arguments. The set specifies one
button in the OptionMenu's Pulldown submenu and some of its resource
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

* **`XmVaSEPARATOR`** 

This is followed by no additional arguments. It specifies one separator
in the OptionMenu's Pulldown submenu.
* **`XmVaDOUBLE_SEPARATOR`** 

This is followed by no additional arguments. It specifies one separator
in the OptionMenu's Pulldown submenu. The separator type is`XmDOUBLE_LINE`.
* **`resource_name`** 

This is followed by one additional argument, the value of the resource,
of type`XtArgVal`. The pair specifies a resource and its value for the
Pulldown submenu.
* **`XtVaTypedArg`** 

This is followed by four additional arguments. The set specifies a
resource and its value for the Pulldown submenu. A resource type
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

This is followed by one additional argument of type`XtVarArgsList`.
This argument is a nested list of`varargs`returned by`XtVaCreateArgsList`.


The user can specify resources in a resource file for the automatically
created widgets and gadgets of an OptionMenu. The following list
identifies the names of these widgets (or gadgets) and the associated
OptionMenu areas:

* **Option Menu Label Gadget** 

`OptionLabel`
* **Option Menu Cascade Button** 

`OptionButton`


For more information on variable-length argument lists, see the X
Toolkit Intrinsics documentation.

A number of resources exist specifically for use with this and
other simple menu creation routines.
For a complete definition of RowColumn and its associated resources, see
&cdeman.XmRowColumn;.
## RETURN


Returns the RowColumn widget ID.
## RELATED


&cdeman.XmCreateOptionMenu;,
&cdeman.XmCreateRowColumn;,
&cdeman.XmCreateSimpleOptionMenu;, and
&cdeman.XmRowColumn;.