# XmDropSiteUpdate
library call`XmDropSiteUpdate`A Drag and Drop function that sets
resource values for a drop siteXmDropSiteUpdateDrag and Drop functionsXmDropSiteUpdate#include <Xm/DragDrop.h>void`XmDropSiteUpdate`WidgetwidgetArgListarglistCardinalargcount
## DESCRIPTION


`XmDropSiteUpdate`modifies drop site resources associated with
the specified widget. This routine updates the drop site resources
specified in the`arglist`.

* **`widget`** 

Specifies the ID of the widget registered as a drop site
* **`arglist`** 

Specifies the argument list
* **`argcount`** 

Specifies the number of attribute/value pairs in the argument
list (`arglist`)


For a complete definition of DropSite and its associated resources,
see &cdeman.XmDropSite;.
## RELATED


&cdeman.XmDropSite;,
&cdeman.XmDropSiteEndUpdate;,
&cdeman.XmDropSiteRegister;,
&cdeman.XmDropSiteStartUpdate;, and
&cdeman.XmDropSiteUnregister;.