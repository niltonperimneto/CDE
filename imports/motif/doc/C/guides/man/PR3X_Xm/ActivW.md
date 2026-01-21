# XmActivateWMProtocol
library call`XmActivateWMProtocol`A VendorShell convenience interface that activates a protocolXmActivateWMProtocolVendorShell functionsXmActivateWMProtocolprotocols#include <Xm/Protocols.h>void`XmActivateWMProtocol`WidgetshellAtomprotocol
## DESCRIPTION


`XmActivateWMProtocol`is a convenience interface.
It calls`XmActivateProtocol`with the property value set to the atom returned by
interningWM_PROTOCOLS.

* **`shell`** 

Specifies the widget with which the protocol property is associated
* **`protocol`** 

Specifies the protocol`Atom`


For a complete definition of VendorShell and its associated resources, see
&cdeman.VendorShell;.
## RELATED


&cdeman.VendorShell;,
&cdeman.XmActivateProtocol;,
&cdeman.XmInternAtom;, and
&cdeman.XmRemoveWMProtocols;.