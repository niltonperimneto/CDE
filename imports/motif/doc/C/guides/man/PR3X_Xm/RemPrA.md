# XmRemoveProtocolCallback
library call`XmRemoveProtocolCallback`A VendorShell function that removes a callback from the internal listXmRemoveProtocol\\%CallbackVendorShell functionsXmRemoveProtocol\\%Callbackprotocols#include <Xm/Xm.h>
#include <Xm/Protocols.h>void`XmRemoveProtocolCallback`WidgetshellAtompropertyAtomprotocolXtCallbackProccallbackXtPointerclosure
## DESCRIPTION


`XmRemoveProtocolCallback`removes a callback from the internal list.

`XmRemoveWMProtocolCallback`is a convenience interface.
It calls`XmRemoveProtocolCallback`with the property value set to the atom returned by
interningWM_PROTOCOLS.

* **`shell`** 

Specifies the widget with which the protocol property is associated
* **`property`** 

Specifies the protocol property
* **`protocol`** 

Specifies the protocol atom
* **`callback`** 

Specifies the procedure to call when a protocol message is received
* **`closure`** 

Specifies the client data to be passed to the callback when it is invoked


For a complete definition of VendorShell and its associated resources, see
&cdeman.VendorShell;.
## RELATED


&cdeman.VendorShell;,
&cdeman.XmAddProtocolCallback;,
&cdeman.XmInternAtom;, and &cdeman.XmRemoveWMProtocolCallback;.