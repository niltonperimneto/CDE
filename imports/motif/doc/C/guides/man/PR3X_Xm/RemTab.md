# XmRemoveTabGroup
library call`XmRemoveTabGroup`A function that removes a tab groupXmRemoveTabGroup#include <Xm/Xm.h>void`XmRemoveTabGroup`Widgettab_group
## DESCRIPTION


This function is obsolete and its behavior is replaced by setting`XmNnavigationType`to`XmNONE`.`XmRemoveTabGroup`removes a
widget from the list of tab groups
associated with a particular widget hierarchy and sets the widget's`XmNnavigationType`to`XmNONE`.

* **`tab_group`** 

Specifies the widget ID

## RELATED


&cdeman.XmAddTabGroup;, &cdeman.XmManager;, and &cdeman.XmPrimitive;.