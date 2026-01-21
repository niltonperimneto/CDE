# XmAddWMProtocols
library call`XmAddWMProtocols`A VendorShell convenience interface that adds the protocols to the protocol manager and allocates the internal tablesXmAddWMProtocol\\%sVendorShell functionsXmAddWMProtocol\\%sprotocols#include <Xm/Protocols.h>void`XmAddWMProtocols`WidgetshellAtom *protocolsCardinalnum_protocols
## DESCRIPTION


`XmAddWMProtocols`is a convenience interface.
It calls`XmAddProtocols`with the property value set to the atom returned by
interningWM_PROTOCOLS.

* **`shell`** 

Specifies the widget with which the protocol property is associated
* **`protocols`** 

Specifies the protocol`Atoms`
* **`num_protocols`** 

Specifies the number of elements in`protocols`


For a complete definition of VendorShell and its associated resources, see
&cdeman.VendorShell;.
## RELATED


&cdeman.VendorShell;,
&cdeman.XmAddProtocols;,
&cdeman.XmInternAtom;, and`XmRemoveWMProtocols`.