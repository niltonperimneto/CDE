# XmMainWindowSetAreas
library call`XmMainWindowSetAreas`A MainWindow function that identifies manageable children for each areaXmMainWindowSetAreasMainWindow functionsXmMainWindowSetAreas#include <Xm/MainW.h>void`XmMainWindowSetAreas`WidgetwidgetWidgetmenu_barWidgetcommand_windowWidgethorizontal_scrollbarWidgetvertical_scrollbarWidgetwork_region
## DESCRIPTION


`XmMainWindowSetAreas`identifies which of the valid children for each
area (such as the MenuBar and work region) are to be actively managed by MainWindow.
This function also sets up or adds the MenuBar, work window, command
window, and ScrollBar widgets to the application's main window widget.

Each area is optional; therefore, the user can pass NULL to one or more
of the following arguments. The window manager provides the title bar.

`NOTE:``XmMainWindowSetAreas`is obsolete and exists for
compatibility with previous releases.
The information previously returned by this function can now be
obtained through a call to`XtGetValues`on the`XmNscrolledWindowChildType`resource.

* **`widget`** 

Specifies the MainWindow widget ID.
* **`menu_bar`** 

Specifies the widget ID for the MenuBar to be associated
with the MainWindow widget. Set this ID only after creating an instance
of the MainWindow widget. The attribute name associated with this
argument is`XmNmenuBar`.
* **`command_window`** 

Specifies the widget ID for the command window
to be associated
with the MainWindow widget. Set this ID only after creating an instance
of the MainWindow widget. The attribute name associated with this
argument is`XmNcommandWindow`.
* **`horizontal_scrollbar`** 

Specifies the ScrollBar widget ID for the
horizontal ScrollBar to be associated
with the MainWindow widget. Set this ID only after creating an instance
of the MainWindow widget. The attribute name associated with this
argument is`XmNhorizontalScrollBar`.
* **`vertical_scrollbar`** 

Specifies the ScrollBar widget ID for the
vertical ScrollBar to be associated
with the MainWindow widget. Set this ID only after creating an instance
of the MainWindow widget. The attribute name associated with this
argument is`XmNverticalScrollBar`.
* **`work_region`** 

Specifies the widget ID for the work window to be associated
with the MainWindow widget. Set this ID only after creating an instance
of the MainWindow widget. The attribute name associated with this
argument is`XmNworkWindow`.


For a complete definition of MainWindow and its associated resources, see
&cdeman.XmMainWindow;.
## RELATED


&cdeman.XmMainWindow;.