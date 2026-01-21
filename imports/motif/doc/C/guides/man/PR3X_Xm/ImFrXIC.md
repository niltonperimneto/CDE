# XmImFreeXIC
library call`XmImFreeXIC`An input manager function that unregisters widgets for an XICXmImFreeXICinput manager functionsXmImFreeXIC#include <Xm/XmIm.h>void`XmImFreeXIC`WidgetwidgetXICxic
## DESCRIPTION


`XmImFreeXIC`unregisters all widgets associated with the specified
X Input Context (XIC).
The specified`widget`must be associated with the specified`xic`.

After unregistering the associated widgets, this call
frees the`xic`.

* **`widget`** 

Specifies the ID of a widget used to identify the`VendorShell`and`XmDisplay`that maintain the widget-XIC registry.
* **`xic`** 

Specifies the Input Context associated with the widget.

## RELATED


&cdeman.XmImGetXIC; and
&cdeman.XmImSetXIC;.