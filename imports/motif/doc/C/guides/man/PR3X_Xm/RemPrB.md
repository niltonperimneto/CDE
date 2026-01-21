# XmRemoveProtocols
library call`XmRemoveProtocols`A VendorShell function that removes the protocols from the protocol manager and deallocates the internal tablesXmRemoveProtocolsVendorShell functionsXmRemoveProtocolsprotocols#include <Xm/Xm.h>
#include <Xm/Protocols.h>void`XmRemoveProtocols`WidgetshellAtompropertyAtom* protocolsCardinalnum_protocols
## DESCRIPTION


`XmRemoveProtocols`removes the protocols from the protocol manager and
deallocates the internal tables. If any of the protocols are active, it
will update the handlers and update the property if`shell`is
realized.

`XmRemoveWMProtocols`is a convenience interface.
It calls`XmRemoveProtocols`with the property value set to the atom returned by
interningWM_PROTOCOLS.

* **`shell`** 

Specifies the widget with which the protocol property is associated
* **`property`** 

Specifies the protocol property
* **`protocols`** 

Specifies the protocol atoms
* **`num_protocols`** 

Specifies the number of elements in protocols


For a complete definition of VendorShell and its associated resources, see
&cdeman.VendorShell;.
## RELATED


&cdeman.VendorShell;,
&cdeman.XmAddProtocols;,
&cdeman.XmInternAtom;, and &cdeman.XmRemoveWMProtocols;.