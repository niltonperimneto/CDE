# XmGetPostedFromWidget
library call`XmGetPostedFromWidget`A RowColumn function that returns the widget from which a menu was postedXmGetPostedFromWidgetRowColumn functionsXmGetPostedFromWidget#include <Xm/RowColumn.h>Widget`XmGetPostedFromWidget`Widgetmenu
## DESCRIPTION


`XmGetPostedFromWidget`returns the widget from which a menu was
posted.
For torn-off menus, this function returns the widget
from which the menu was originally torn.
An application can use this routine during the activate callback to
determine the context in which the menu callback should be interpreted.

* **`menu`** 

Specifies the widget ID of the menu


For a complete definition of RowColumn and its associated resources, see
&cdeman.XmRowColumn;.
## RETURN


Returns the widget ID of the widget from which the menu was posted.
If the menu is a Popup Menu, the returned widget is the widget from
which the menu was popped up.
If the menu is a Pulldown Menu, the returned widget is the MenuBar or
OptionMenu from which the widget was pulled down.
## RELATED


&cdeman.XmRowColumn;.