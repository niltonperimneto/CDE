# XmCreateMenuBar
library call`XmCreateMenuBar`A RowColumn widget convenience creation functionXmCreateMenuBarcreation functionsXmCreateMenuBar#include <Xm/RowColumn.h>Widget`XmCreateMenuBar`WidgetparentStringnameArgListarglistCardinalargcount
## DESCRIPTION


`XmCreateMenuBar`creates an instance of a RowColumn widget
of type`XmMENU_BAR`and returns the
associated widget ID. It is provided as a convenience
function for creating RowColumn
widgets configured to operate as a MenuBar and is not implemented as a
separate widget class.

The MenuBar widget is generally used for building a Pulldown
menu system. Typically, a MenuBar is created and placed along the
top of the application window, and several CascadeButtons
are inserted as the children. Each of the CascadeButtons has a
Pulldown menu pane associated with it.
These Pulldown menu panes must have been created as children of the MenuBar.
The user interacts with the MenuBar by using either the mouse or
the keyboard.

The MenuBar displays a 3-D shadow along its border. The application
controls the shadow attributes using the visual-related resources
supported by`XmManager`.

The MenuBar widget is homogeneous in that it accepts only children
that are a subclass of`XmCascadeButton`or`XmCascadeButtonGadget`.
Attempting to insert a child of a different class results in a warning
message.

If the MenuBar does not have enough room to fit all of its subwidgets on a
single line, the MenuBar attempts to wrap the remaining entries onto
additional lines if allowed by the geometry manager of the parent widget.

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


&cdeman.XmCascadeButton;,
&cdeman.XmCascadeButtonGadget;,
&cdeman.XmCreatePulldownMenu;,
&cdeman.XmCreateSimpleMenuBar;,
&cdeman.XmManager;,
&cdeman.XmRowColumn;, and
&cdeman.XmVaCreateSimpleMenuBar;.