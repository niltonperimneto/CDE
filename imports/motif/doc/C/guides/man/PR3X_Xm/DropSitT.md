# XmDropSiteRegister
library call`XmDropSiteRegister`A Drag and Drop function that identifies
a drop site and assigns resources that specify its behaviorXmDropSiteRegisterDrag and Drop functionsXmDropSiteRegisterregister functionsXmDropSiteRegister#include <Xm/DragDrop.h>void`XmDropSiteRegister`WidgetwidgetArgListarglistCardinalargcount
## DESCRIPTION


`XmDropSiteRegister`identifies the specified widget or
gadget as a drop site and sets resource values that define
the drop site's behavior. The routine assigns default values
to any resources that are not specified in the argument list.
The toolkit generates a warning message if a drop site is
registered with`XmNdropSiteActivity`set to`XmDROP_SITE_ACTIVE`and the`XmNdropProc`resource
is NULL.

If the drop site is a descendant of a widget that is registered
as a drop site, the`XmNdropSiteType`resource of the ancestor
drop site must be specified as`XmDROP_SITE_COMPOSITE`. The
ancestor must be registered before the descendant. The drop site
is stacked above all other sibling drop sites already registered.

* **`widget`** 

Specifies the ID of the widget to be registered.
* **`arglist`** 

Specifies the argument list.
* **`argcount`** 

Specifies the number of attribute/value pairs in the argument
list (`arglist`).


For a complete definition of DropSite and its associated resources,
see &cdeman.XmDropSite;.
## RELATED


&cdeman.XmDisplay;,
&cdeman.XmDropSite;,
&cdeman.XmDropSiteEndUpdate;,
&cdeman.XmDropSiteStartUpdate;,
&cdeman.XmDropSiteUpdate;,
&cdeman.XmDropSiteUnregister;, and
&cdeman.XmScreen;.