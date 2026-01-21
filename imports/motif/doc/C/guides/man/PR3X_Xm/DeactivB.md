# XmDeactivateWMProtocol
library call`XmDeactivateWMProtocol`A VendorShell convenience interface that deactivates a protocol without removing itprotocolsXmDeactivateWMProtocolVendorShell functionsXmDeactivateWMProtocol#include <Xm/Xm.h>
#include <Xm/Protocols.h>void`XmDeactivateWMProtocol`WidgetshellAtomprotocol
## DESCRIPTION


`XmDeactivateWMProtocol`is a convenience interface.
It calls`XmDeactivateProtocol`with the property value set to the atom returned by
interningWM_PROTOCOLS.

* **`shell`** 

Specifies the widget with which the protocol property is associated
* **`protocol`** 

Specifies the protocol atom


For a complete definition of VendorShell and its associated resources, see
&cdeman.VendorShell;.
## RELATED


&cdeman.VendorShell;,
&cdeman.XmActivateWMProtocol;,
&cdeman.XmDeactivateProtocol;, and
&cdeman.XmInternAtom;.