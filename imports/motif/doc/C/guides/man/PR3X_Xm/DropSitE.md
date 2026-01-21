# XmDropSiteEndUpdate
library call`XmDropSiteEndUpdate`A Drag and Drop function that facilitates
processing updates to multiple drop sitesXmDropSiteEndUpdateDrag and Drop functionsXmDropSiteEndUpdate#include <Xm/DragDrop.h>void`XmDropSiteEndUpdate`Widgetwidget
## DESCRIPTION


`XmDropSiteEndUpdate`is used in conjunction with`XmDropSiteStartUpdate`to process updates to
multiple drop sites within the same hierarchy.`XmDropSiteStartUpdate`and`XmDropSiteEndUpdate`signal the beginning and
the end respectively of a series of calls to`XmDropSiteUpdate`.
Calls to`XmDropSiteStartUpdate`and`XmDropSiteEndUpdate`can
be recursively stacked. Using these routines optimizes the processing
of update information.

* **`widget`** 

Specifies the ID of any widget within a given hierarchy. The function
uses this widget to identify the shell that contains the drop sites.


For a complete definition of DropSite and its associated resources,
see &cdeman.XmDropSite;.
## RELATED


&cdeman.XmDropSiteStartUpdate; and
&cdeman.XmDropSiteUpdate;.