# XmAddProtocols
library call`XmAddProtocols`A VendorShell function that adds the protocols to the protocol manager and allocates the internal tablesXmAddProtocolsVendorShell functionsXmAddProtocolsprotocols#include <Xm/Protocols.h>void`XmAddProtocols`WidgetshellAtompropertyAtom *protocolsCardinalnum_protocols
## DESCRIPTION


`XmAddProtocols`adds the protocols to the
protocol manager and allocates the internal tables.

`XmAddWMProtocols`is a convenience interface.
It calls`XmAddProtocols`with the property value set to the atom returned by
interningWM_PROTOCOLS.

* **`shell`** 

Specifies the widget with which the protocol property is associated
* **`property`** 

Specifies the protocol property
* **`protocols`** 

Specifies the protocol`Atoms`
* **`num_protocols`** 

Specifies the number of elements in`protocols`


For a complete definition of VendorShell and its associated resources, see
&cdeman.VendorShell;.
## RELATED


&cdeman.VendorShell;,
&cdeman.XmAddWMProtocols;,
&cdeman.XmInternAtom;, and
&cdeman.XmRemoveProtocols;.