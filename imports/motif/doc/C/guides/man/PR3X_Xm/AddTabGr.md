# XmAddTabGroup
library call`XmAddTabGroup`A function that adds a manager or a primitive
widget to the list of tab groupsXmAddTabGroupVendorShell functionsXmAddTabGroupprotocols#include <Xm/Xm.h>void`XmAddTabGroup`Widgettab_group
## DESCRIPTION


This function is obsolete and its behavior is replaced by setting`XmNnavigationType`to`XmEXCLUSIVE_TAB_GROUP`.
When the keyboard is used to traverse through a widget hierarchy, primitive
or manager
widgets are grouped together into what are known as`tab groups`. Any
manager or primitive widget can be a tab group. Within
a tab group, move the focus to the next widget in the tab
group by using the arrow keys. To move to another tab group, use`KNextField`or`KPrevField`.

Tab groups are ordinarily specified by the`XmNnavigationType`resource.`XmAddTabGroup`is called to control the order of traversal of tab
groups. The widget specified
by`tab_group`is appended to the list of tab
groups to be traversed, and the widget's`XmNnavigationType`is set
to`XmEXCLUSIVE_TAB_GROUP`.

* **`tab_group`** 

Specifies the manager or primitive widget ID

## RELATED


&cdeman.XmManager;,
&cdeman.XmGetTabGroup;,
&cdeman.XmPrimitive;, and
&cdeman.XmRemoveTabGroup;.