# XmContainerGetItemChildren
library call`XmContainerGetItemChildren`Container widget function to find
all children of an itemXmContainerGetItemChildrenXmContainer#include <Xm/Container.h>int`XmContainerGetItemChildren`WidgetcontainerWidgetitemWidgetList *item_children
## DESCRIPTION


`XmContainerGetItemChildren`allocates a WidgetList and stores within
it the widget IDs of all widgets that have`item`specified as the value
of their`XmNentryParent`resource.
The application programmer is responsible for freeing the allocated
WidgetList using XtFree.
The number of widget IDs returned
in`item_children`is returned by the function. If no widgets specify`item`as the value of their`XmNentryParent`resource,
the function returns zero and`item_children`is left unchanged.

* **`container`** 

Specifies the Container widget ID.
* **`item`** 

Specifies a widgetID within`container`.
* **`item_children`** 

Returned array of Widgets.


For a complete definition of Container and its associated resources, see
&cdeman.XmContainer;.
## RETURN


This function returns a count of all widgets that have`item`specified as the value of their`XmNentryParent`resource.
## RELATED


&cdeman.XmContainer;.