# XmDropSiteRegistered
library call`XmDropSiteRegistered`A Drag and Drop function that determines if a drop site has been registeredXmDropSiteRegisteredDrag and Drop functionsXmDropSiteRegisteredregister functionsXmDropSiteRegistered#include <Xm/DragDrop.h>Boolean`XmDropSiteRegistered`Widgetwidget
## DESCRIPTION


`XmDropSiteRegistered`determines if the specified widget has a
drop site registered. If a drop site is registered, this function returns
True.

* **`widget`** 

Specifies the ID of the widget being queried.


For a complete definition of DropSite and its associated resources,
see &cdeman.XmDropSite;.
## RETURN


If the widget is not a registered drop site, this function returns
False. Otherwise, it returns True.
## RELATED


&cdeman.XmDisplay;,
&cdeman.XmDropSite;,
&cdeman.XmDropSiteEndUpdate;,
&cdeman.XmDropSiteStartUpdate;,
&cdeman.XmDropSiteUpdate;,
&cdeman.XmDropSiteUnregister;, and
&cdeman.XmScreen;.