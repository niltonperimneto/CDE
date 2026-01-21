# XmAddProtocolCallback
library call`XmAddProtocolCallback`A VendorShell function that adds client callbacks for a protocolXmAddProtocolCallbackVendorShell functionsXmAddProtocolCallbackprotocols#include <Xm/Protocols.h>void`XmAddProtocolCallback`WidgetshellAtompropertyAtomprotocolXtCallbackProccallbackXtPointerclosure
## DESCRIPTION


`XmAddProtocolCallback`adds client callbacks for a protocol. It
checks if the protocol is registered, and if it is not,
calls`XmAddProtocols`. It then adds the callback to
the internal list. These callbacks are called when
the corresponding client message is received.

`XmAddWMProtocolCallback`is a convenience interface.
It calls`XmAddProtocolCallback`with the property value set to the atom returned by
interningWM_PROTOCOLS.

* **`shell`** 

Specifies the widget with which the protocol property is associated
* **`property`** 

Specifies the protocol property
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
&cdeman.XmAddWMProtocolCallback;,
&cdeman.XmInternAtom;, and
&cdeman.XmRemoveProtocolCallback;.