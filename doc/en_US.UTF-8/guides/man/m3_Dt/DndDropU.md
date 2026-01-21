# DtDndDropUnregister
library call`DtDndDropUnregister`deactivate a drop site#include <Dt/Dnd.h>void`DtDndDropUnregister`WidgetdropSite
## DESCRIPTION


The`DtDndDropUnregister`function removes the widget,dropSite, from the set of drop sites that have been registered with
&cdeman.DtDndDropRegister; and frees data allocated by a call to
&cdeman.DtDndDropRegister;.

The`DtDndDropUnregister`function
is used to unregister a widget when it is no longer a drop site.
A widget can be unregistered with`DtDndDropUnregister`at any time after it has been registered with
&cdeman.DtDndDropRegister;.
## RETURN VALUE


The`DtDndDropUnregister`function returns no value.
## SEE ALSO


&cdeman.Dt.Dnd.h;, &cdeman.DtDndDropRegister;,