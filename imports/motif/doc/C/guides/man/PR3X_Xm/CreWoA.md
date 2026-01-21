# XmCreateWorkArea
library call`XmCreateWorkArea`A function that creates a RowColumn WorkAreaXmCreateWorkAreacreation functionsXmCreateWorkArea#include <Xm/RowColumn.h>Widget`XmCreateWorkArea`WidgetparentStringnameArgListarglistCardinalargcount
## DESCRIPTION


`XmCreateWorkArea`creates an instance of a
RowColumn widget and returns the associated widget ID.
The widget is created with`XmNrowColumnType`set to`XmWORK_AREA`.

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


&cdeman.XmCreateRadioBox;,
&cdeman.XmCreateSimpleCheckBox;,
&cdeman.XmCreateSimpleRadioBox;,
&cdeman.XmRowColumn;,
&cdeman.XmVaCreateSimpleCheckBox;, and
&cdeman.XmVaCreateSimpleRadioBox;.