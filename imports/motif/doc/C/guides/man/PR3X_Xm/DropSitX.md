# XmDropSiteUnregister
library call`XmDropSiteUnregister`A Drag and Drop function that
frees drop site informationXmDropSiteUnregisterDrag and Drop functionsXmDropSiteUnregister#include <Xm/DragDrop.h>void`XmDropSiteUnregister`Widgetwidget
## DESCRIPTION


`XmDropSiteUnregister`informs the toolkit that the specified
widget is no longer a registered drop site. The function frees all
associated drop site information.

* **`widget`** 

Specifies the ID of the widget, registered as a drop site,
that is to be unregistered


For a complete definition of DropSite and its associated resources,
see &cdeman.XmDropSite;.
## RELATED


&cdeman.XmDropSite; and
&cdeman.XmDropSiteRegister;.