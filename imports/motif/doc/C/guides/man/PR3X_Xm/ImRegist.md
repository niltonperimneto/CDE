# XmImRegister
library call`XmImRegister`An input manager function that registers a widget with an input managerXmImRegisterinput manager functionsXmImRegister#include <Xm/XmIm.h>void`XmImRegister`Widgetwidgetunsigned intreserved
## DESCRIPTION


`XmImRegister`registers a widget with its input manager.
This adds the specified widget to a list of widgets that
are supported by the input manager for an input method.
If an input method has not been opened
by a previous call to`XmImRegister`, the first time this
routine is called it opens an input method using the`XmNinputMethod`resource for the VendorShell. If the`XmNinputMethod`is NULL, an input method is opened using the
current locale.

If an input method cannot be opened in
the current locale,`XLookupString`provides input processing.

The application is responsible for unregistering a widget by calling`XmImUnregister`.

Note that the Text, TextField, and List widgets already call the`XmImRegister`function internally. You should not call this function for
these widgets before calling`XmImUnregister`first.

* **`widget`** 

Specifies the ID of the widget to be registered.
* **`reserved`** 

This argument is not used in the current release of Motif.
The value should always be 0 (zero).

## RELATED


&cdeman.XmImGetXIM;,
&cdeman.XmImMbLookupString;,
and &cdeman.XmImUnregister;.