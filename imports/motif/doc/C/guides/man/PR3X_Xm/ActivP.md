# XmActivateProtocol
library call`XmActivateProtocol`A VendorShell function that activates a protocolXmActivateProtocolVendorShell functionsXmActivateProtocolprotocols#include <Xm/Protocols.h>void`XmActivateProtocol`WidgetshellAtompropertyAtomprotocol
## DESCRIPTION


`XmActivateProtocol`activates a protocol. It
updates the handlers and the`property`if
the`shell`is realized. It is sometimes useful to allow
a protocol's state information (callback lists, and so on) to persist, even though
the client may choose to temporarily resign from the interaction. This is
supported by allowing a`protocol`to be in one of two states: active or
inactive. If the`protocol`is active and the`shell`is realized,
the`property`contains the`protocol``Atom`.
If the`protocol`is inactive, the`Atom`is not present in
the`property`.

`XmActivateWMProtocol`is a convenience interface.
It calls`XmActivateProtocol`with the property value set to the atom returned by
interningWM_PROTOCOLS.

* **`shell`** 

Specifies the widget with which the protocol property is associated
* **`property`** 

Specifies the protocol property
* **`protocol`** 

Specifies the protocol`Atom`


For a complete definition of VendorShell and its associated resources, see
&cdeman.VendorShell;.
## RELATED


&cdeman.VendorShell;, &cdeman.XmActivateWMProtocol;,
&cdeman.XmRemoveProtocols;
and &cdeman.XmInternAtom;.