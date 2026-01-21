# XmMenuPosition
library call`XmMenuPosition`A RowColumn function that positions a Popup menu paneXmMenuPositionRowColumn functionsXmMenuPosition#include <Xm/RowColumn.h>void`XmMenuPosition`WidgetmenuXButtonPressedEvent* event
## DESCRIPTION


`XmMenuPosition`positions a Popup menu pane using the information in the
specified event. Unless an application is positioning the menu pane
itself, it must first invoke this function before managing the PopupMenu.
The`x_root`and`y_root`fields
in the specified
X
event are used to
determine the menu position.

* **`menu`** 

Specifies the PopupMenu to be positioned
* **`event`** 

Specifies the event passed to the action procedure which manages the
PopupMenu


Which corner of the PopupMenu is positioned at the`x_root`and`y_root`depends on the`XmNlayoutDirection`resource of the widget from
which popup occurs.

For a complete definition of RowColumn and its associated resources, see
&cdeman.XmRowColumn;.
## RELATED


&cdeman.XmRowColumn;.