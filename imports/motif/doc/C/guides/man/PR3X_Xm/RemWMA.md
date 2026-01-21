# XmRemoveWMProtocolCallback
library call`XmRemoveWMProtocolCallback`A VendorShell convenience interface that removes a callback from the internal listXmRemoveWMProtocol\\%CallbackVendorShell functionsXmRemoveWMProtocol\\%Callbackprotocols#include <Xm/Xm.h>
#include <Xm/Protocols.h>void`XmRemoveWMProtocolCallback`WidgetshellAtomprotocolXtCallbackProccallbackXtPointerclosure
## DESCRIPTION


`XmRemoveWMProtocolCallback`is a convenience interface.
It calls`XmRemoveProtocolCallback`with the property value set to the atom returned by
interningWM_PROTOCOLS.

* **`shell`** 

Specifies the widget with which the protocol property is associated
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
&cdeman.XmAddWMProtocolCallback;,
&cdeman.XmInternAtom;, and &cdeman.XmRemoveProtocolCallback;.