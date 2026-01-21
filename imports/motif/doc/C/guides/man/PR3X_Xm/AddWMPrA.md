# XmAddWMProtocolCallback
library call`XmAddWMProtocolCallback`A VendorShell convenience interface that adds client callbacks for a protocolXmAddWMProtocol\\%CallbackVendorShell functionsXmAddWMProtocol\\%Callbackprotocols#include <Xm/Protocols.h>void`XmAddWMProtocolCallback`WidgetshellAtomprotocolXtCallbackProccallbackXtPointerclosure
## DESCRIPTION


`XmAddWMProtocolCallback`is a convenience interface.
It calls`XmAddProtocolCallback`with the property value set to the atom returned by
interningWM_PROTOCOLS.

* **`shell`** 

Specifies the widget with which the protocol property is associated
* **`protocol`** 

Specifies the protocol`Atom`
* **`callback`** 

Specifies the procedure to call when a protocol message is received
* **`closure`** 

Specifies the client data to be passed to the callback when it is invoked


For a complete definition of VendorShell and its associated resources, see
&cdeman.VendorShell;.
## RELATED


&cdeman.VendorShell;,
&cdeman.XmAddProtocolCallback;,
&cdeman.XmInternAtom;, and
&cdeman.XmRemoveWMProtocolCallback;.