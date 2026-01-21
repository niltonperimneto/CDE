# XmCreatePopupMenu
library call`XmCreatePopupMenu`A RowColumn widget convenience creation functionXmCreatePopupMenucreation functionsXmCreatePopupMenu#include <Xm/RowColumn.h>Widget`XmCreatePopupMenu`WidgetparentStringnameArgListarglistCardinalargcount
## DESCRIPTION


`XmCreatePopupMenu`creates an instance of a RowColumn widget
of type`XmMENU_POPUP`and returns
the associated widget ID. When this function is used to create the Popup
menu pane, a MenuShell widget is automatically created as the parent of
the menu pane.
The parent of the MenuShell widget is the widget indicated by the`parent`parameter.

`XmCreatePopupMenu`is
provided as a convenience function for creating RowColumn
widgets configured to operate as Popup menu panes and is not implemented as
a separate widget class.

The PopupMenu is used as the first menu pane within a PopupMenu system;
all other
menu panes are of the Pulldown type. A Popup menu pane displays a
3-D shadow, unless the feature is disabled by the application.
The shadow appears around the edge of the menu pane.

The Popup menu pane must be created as the child of a MenuShell widget in
order to function properly when it is incorporated into a menu.
If the application uses this convenience function for creating a
Popup menu pane, the MenuShell is automatically created as the real
parent of the menu pane.
If the application does not use this convenience function to create the
RowColumn to function as a Popup menu pane,
it is the application's responsibility to create the
MenuShell widget.

To access the PopupMenu, the application must first position the
widget using the`XmMenuPosition`function and then manage it using`XtManageChild`.

* **`parent`** 

Specifies the parent widget ID
* **`name`** 

Specifies the name of the created widget
* **`arglist`** 

Specifies the argument list
* **`argcount`** 

Specifies the number of attribute/value pairs in the argument list
(`arglist`)


Popup menu panes support tear-off capabilities
for tear-off menus through`XmRowColumn`resources.
For a complete definition of RowColumn and its associated resources, see
&cdeman.XmRowColumn;.
## RETURN


Returns the RowColumn widget ID.
## RELATED


&cdeman.XmCreateSimplePopupMenu;,
&cdeman.XmMenuPosition;,
&cdeman.XmMenuShell;,
&cdeman.XmRowColumn;, and
&cdeman.XmVaCreateSimplePopupMenu;.