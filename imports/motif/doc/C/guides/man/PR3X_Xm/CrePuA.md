# XmCreatePulldownMenu
library call`XmCreatePulldownMenu`A RowColumn widget convenience creation functionXmCreatePulldownMenucreation functionsXmCreatePulldownMenu#include <Xm/RowColumn.h>Widget`XmCreatePulldownMenu`WidgetparentStringnameArgListarglistCardinalargcount
## DESCRIPTION


`XmCreatePulldownMenu`creates an instance of a
RowColumn widget of type`XmMENU_PULLDOWN`and returns
the associated widget ID.

* **`parent`** 

Specifies the parent widget ID
* **`name`** 

Specifies the name of the created widget
* **`arglist`** 

Specifies the argument list
* **`argcount`** 

Specifies the number of attribute/value pairs in the argument list
(`arglist`)


Specifies the number of attribute/value pairs in the argument list
(`arglist`).
When this function is used to create the Pulldown
menu pane, a MenuShell widget is automatically created as the parent of the
menu pane. If the widget specified
by the`parent`parameter is a
Popup or a Pulldown menu pane, the MenuShell
widget is created as a child of the`parent`MenuShell; otherwise, it
is created as a child of the specified`parent`widget.

`XmCreatePulldownMenu`is
provided as a convenience function for creating RowColumn
widgets configured to operate as Pulldown menu panes and is not implemented as
a separate widget class.

A Pulldown menu pane displays a
3-D shadow, unless the feature is disabled by the application.
The shadow appears around the edge of the menu pane.

A Pulldown menu pane is used with submenus that are to be
attached to a CascadeButton or a
CascadeButtonGadget. This is the case for all menu panes
that are part of a PulldownMenu system (a MenuBar), the menu pane
associated with an OptionMenu, and any menu panes that cascade from
a Popup menu pane. Pulldown menu panes that are to be associated with an
OptionMenu must be created before the OptionMenu is created.

The Pulldown menu pane must be attached to a CascadeButton or CascadeButtonGadget
that resides in a MenuBar, a Popup menu pane, a Pulldown menu pane, or an
OptionMenu. It is attached with the button resource`XmNsubMenuId`.

A MenuShell widget is required between the Pulldown menu pane and its
parent.
If the application uses this convenience function for creating a
Pulldown menu pane, the MenuShell is automatically created as the real
parent of the menu pane; otherwise,
it is the application's responsibility to create
the MenuShell widget.

To function correctly when incorporated into a menu, the Pulldown menu pane's
hierarchy must be considered. This hierarchy depends on the
type of menu system that is being built, as follows:

If the Pulldown menu pane is to be pulled down from a MenuBar, its`parent`must be the MenuBar.

If the Pulldown menu pane is to be pulled down from a Popup or another
Pulldown menu pane, its`parent`must be that Popup or Pulldown menu pane.

If the Pulldown menu pane is to be pulled down from an OptionMenu, its`parent`must be the same as the OptionMenu parent.

PullDown menu panes support tear-off capabilities
for tear-off menus through`XmRowColumn`resources.
For a complete definition of RowColumn and its associated resources, see
&cdeman.XmRowColumn;.
## RETURN


Returns the RowColumn widget ID.
## RELATED


&cdeman.XmCascadeButton;,
&cdeman.XmCascadeButtonGadget;,
&cdeman.XmCreateOptionMenu;,
&cdeman.XmCreatePopupMenu;,
&cdeman.XmCreateSimplePulldownMenu;,
&cdeman.XmMenuShell;,
&cdeman.XmRowColumn;, and
&cdeman.XmVaCreateSimplePulldownMenu;.