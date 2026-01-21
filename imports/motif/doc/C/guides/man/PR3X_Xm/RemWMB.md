# XmRemoveWMProtocols
library call`XmRemoveWMProtocols`A VendorShell convenience interface that removes the protocols from the protocol manager and deallocates the internal tablesXmRemoveWMProtocolsVendorShell functionsXmRemoveWMProtocolsprotocols#include <Xm/Xm.h>
#include <Xm/Protocols.h>void`XmRemoveWMProtocols`WidgetshellAtom* protocolsCardinalnum_protocols
## DESCRIPTION


`XmRemoveWMProtocols`is a convenience interface.
It calls`XmRemoveProtocols`with the property value set to the atom returned by
interningWM_PROTOCOLS.

* **`shell`** 

Specifies the widget with which the protocol property is associated
* **`protocols`** 

Specifies the protocol atoms
* **`num_protocols`** 

Specifies the number of elements in protocols


For a complete definition of VendorShell and its associated resources, see
&cdeman.VendorShell;.
## RELATED


&cdeman.VendorShell;,
&cdeman.XmAddWMProtocols;,
&cdeman.XmInternAtom;, and &cdeman.XmRemoveProtocols;.