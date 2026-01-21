# XmCreateOptionMenu
library call`XmCreateOptionMenu`A RowColumn widget convenience creation functionXmCreateOptionMenucreation functionsXmCreateOptionMenu#include <Xm/RowColumn.h>Widget`XmCreateOptionMenu`WidgetparentStringnameArgListarglistCardinalargcount
## DESCRIPTION


`XmCreateOptionMenu`creates an instance of a
RowColumn widget of type`XmMENU_OPTION`and returns the associated widget ID.

It is provided as a convenience function for creating a RowColumn
widget configured to operate as an OptionMenu and is not implemented as a
separate widget class.

The OptionMenu widget is a specialized RowColumn manager composed of a
label, a selection area, and a
single Pulldown menu pane. When an application creates
an OptionMenu widget, it supplies the label string and the Pulldown menu pane.
In order for the operation to be successful,
there must be a valid`XmNsubMenuId`resource set
when this function is called.
The LabelGadget and the selection area (a CascadeButtonGadget) are created
by the OptionMenu.

The OptionMenu's Pulldown menu pane must not contain any ToggleButtons or
ToggleButtonGadgets.
The results of including CascadeButtons or CascadeButtonGadgets in the
OptionMenu's Pulldown menu pane are undefined.

An OptionMenu is laid out with the label displayed on one side of
the widget and the selection area on the other side when`XmNorientation`is`XmHORIZONTAL`.
The layout of the label with respect to the selection area depends on
the`XmNlayoutDirection`resource in the horizontal orientation.
If the value is`XmVERTICAL`,
the label is above the selection area.
The selection area has a dual purpose; it displays the label of the last
item selected from the associated Pulldown menu pane, and it provides
the means for posting the Pulldown menu pane.

The OptionMenu typically does not display any 3-D visuals around
itself or the internal LabelGadget. By default, the internal
CascadeButtonGadget has a visible 3-D shadow.
The application may change this
by getting the CascadeButtonGadget ID using`XmOptionButtonGadget`, and then
calling`XtSetValues`using the standard visual-related resources.

The Pulldown menu pane is posted when the mouse pointer is moved over the
selection area and a mouse button that is defined by OptionMenu's
RowColumn parent is pressed.
The Pulldown menu pane is posted
and positioned so that the last selected item is directly over
the selection area. The mouse is then used to arm the desired menu
item. When the mouse button is released, the armed menu item is selected
and the label within the selection area is changed to match that of
the selected item. By default,`BSelect`is used to interact with an
OptionMenu.
The default can be changed with the RowColumn resource`XmNmenuPost`.

The OptionMenu also operates with the keyboard interface mechanism.
If the application has established a mnemonic with the OptionMenu,
pressingAltwith the mnemonic causes the Pulldown menu pane to be posted with
traversal enabled.
The standard traversal keys can then be used to
move within the menu pane. PressingReturnor typing a mnemonic or accelerator for one of the
menu items selects that item.

An application may use the`XmNmenuHistory`resource to
indicate which item in the Pulldown menu pane should be treated as the current
choice and have its label displayed in the selection area. By default,
the first selectable item in the Pulldown menu pane is used.

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


For a complete definition of RowColumn and its associated resources, see
&cdeman.XmRowColumn;.
## RETURN


Returns the RowColumn widget ID.
## RELATED


&cdeman.XmCascadeButtonGadget;,
&cdeman.XmCreatePulldownMenu;,
&cdeman.XmCreateSimpleOptionMenu;,
&cdeman.XmLabelGadget;,
&cdeman.XmOptionButtonGadget;,
&cdeman.XmOptionLabelGadget;,
&cdeman.XmRowColumn;, and
&cdeman.XmVaCreateSimpleOptionMenu;.