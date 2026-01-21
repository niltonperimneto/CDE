# XmContainerReorder
library call`XmContainerReorder`Container widget function to reorder childrenXmContainerReorderXmContainer#include <Xm/Container.h>void`XmContainerReorder`WidgetcontainerWidgetListwidgetsintnum_widgets
## DESCRIPTION


`XmContainerReorder`obtains the`XmNpositionIndex`constraint
resources of each widget specified in`widgets`, sorts them in
ascending order, and inserts the`XmNpositionIndex`constraint
resources in the new order into each widget.
If the`XmNlayoutType`resource of Container is`XmOUTLINE`or`XmDETAIL`,`XmContainerReorder`will force a layout of all
items.

* **`container`** 

Specifies the Container widget ID.
* **`widgets`** 

Specifies an array of widget children of`container`.
* **`num_widgets`** 

Specifies the number of items in the`widgets`array.


For a complete definition of Container and its associated resources, see
&cdeman.XmContainer;.
## RELATED


&cdeman.XmContainer;.