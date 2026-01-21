# XmGetTabGroup
library call`XmGetTabGroup`Returns the widget ID of a tab groupXmGetTabGrouptraversal functionsXmGetTabGroup#include <Xm/Xm.h>Widget`XmGetTabGroup`Widgetwidget
## DESCRIPTION


`XmGetTabGroup`returns the widget ID of the tab group that
contains the specified widget.

* **`widget`** 

Specifies a widget ID within a tab group

## RETURN


Returns the widget ID of a tab group or shell, determined as follows:

If`widget`is a tab group or shell, returns`widget`

If neither`widget`nor any ancestor up to the nearest shell is a tab
group, returns the nearest ancestor of`widget`that is a shell

Otherwise, returns the nearest ancestor of`widget`that is a tab group
## RELATED


&cdeman.XmAddTabGroup;,
&cdeman.XmManager;, and
&cdeman.XmPrimitive;.