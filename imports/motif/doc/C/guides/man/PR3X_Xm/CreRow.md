# XmCreateRowColumn
library call`XmCreateRowColumn`The RowColumn widget creation functionXmCreateRowColumncreation functionsXmCreateRowColumn#include <Xm/RowColumn.h>Widget`XmCreateRowColumn`WidgetparentStringnameArgListarglistCardinalargcount
## DESCRIPTION


`XmCreateRowColumn`creates an instance of a
RowColumn widget and returns the associated widget ID. If`XmNrowColumnType`is not specified, then it is created with`XmWORK_AREA`, which is the default.

If this function is used to create a Popup Menu of type`XmMENU_POPUP`or a Pulldown Menu of type`XmMENU_PULLDOWN`,
a MenuShell widget is not automatically created as the parent of the
menu pane. The application must first create the MenuShell by using either`XmCreateMenuShell`or the standard toolkit create function.

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


&cdeman.XmCreateMenuBar;,
&cdeman.XmCreateMenuShell;,
&cdeman.XmCreateOptionMenu;,
&cdeman.XmCreatePopupMenu;,
&cdeman.XmCreatePulldownMenu;,
&cdeman.XmCreateRadioBox;,
&cdeman.XmCreateSimpleCheckBox;,
&cdeman.XmCreateSimpleMenuBar;,
&cdeman.XmCreateSimpleOptionMenu;,
&cdeman.XmCreateSimplePopupMenu;,
&cdeman.XmCreateSimplePulldownMenu;,
&cdeman.XmCreateSimpleRadioBox;,
&cdeman.XmCreateWorkArea;,
&cdeman.XmRowColumn;,
&cdeman.XmVaCreateSimpleCheckBox;,
&cdeman.XmVaCreateSimpleMenuBar;,
&cdeman.XmVaCreateSimpleOptionMenu;,
&cdeman.XmVaCreateSimplePopupMenu;,
&cdeman.XmVaCreateSimplePulldownMenu;, and
&cdeman.XmVaCreateSimpleRadioBox;.