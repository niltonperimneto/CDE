# XmImSetXIC
library call`XmImSetXIC`An input manager function that registers an existing XIC with a widgetXmImSetXICinput manager functionsXmImSetXIC#include <Xm/XmIm.h>XIC`XmImSetXIC`WidgetwidgetXICxic
## DESCRIPTION


`XmImSetXIC`registers the specified X Input Context (XIC) with`widget`. Any existing XIC registered for`widget`is
unregistered. The new XIC registered for`widget`is returned.

If`xic`was not created by`XmImGetXIC`or`XmImRegister`, it will not be subject to closing activities when it
has no widgets registered with it.

* **`widget`** 

Specifies the ID of a widget for which a new Input Context is to be
registered.
* **`xic`** 

Specifies the Input Context to be registered with the widget.
If`xic`is NULL, the function returns the current`XIC`used by`widget`.

## RETURN


Returns the new XIC registered for`widget`.
The application is responsible for freeing the returned XIC.
To free the XIC, call`XmImFreeXIC`.
## RELATED


&cdeman.XmImGetXIC; and
&cdeman.XmImFreeXIC;.