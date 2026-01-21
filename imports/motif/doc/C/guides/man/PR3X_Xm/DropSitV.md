# XmDropSiteRetrieve
library call`XmDropSiteRetrieve`A Drag and Drop function that retrieves
resource values set on a drop siteXmDropSiteRetrieveDrag and Drop functionsXmDropSiteRetrieve#include <Xm/DragDrop.h>void`XmDropSiteRetrieve`WidgetwidgetArgListarglistCardinalargcount
## DESCRIPTION


`XmDropSiteRetrieve`extracts values for the given
resources from the drop site specified by`widget`.
An initiator can also obtain information about the current drop
site by passing the associated DragContext widget as the`widget`parameter to this routine. The initiator can retrieve all
of the drop site resources except`XmNdragProc`and`XmNdropProc`using this method.

* **`widget`** 

Specifies the ID of the widget that encloses the drop site.
* **`arglist`** 

Specifies the argument list.
* **`argcount`** 

Specifies the number of attribute/value pairs in the argument
list (`arglist`).


For a complete definition of DropSite and its associated resources,
see &cdeman.XmDropSite;.
## RELATED


&cdeman.XmDropSite; and
&cdeman.XmDropSiteUpdate;.