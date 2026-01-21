# XmDeactivateProtocol
library call`XmDeactivateProtocol`A VendorShell function that deactivates a protocol without removing itXmDeactivateProtocolprotocolsVendorShell functionsXmDeactivateProtocol#include <Xm/Xm.h>
#include <Xm/Protocols.h>void`XmDeactivateProtocol`WidgetshellAtompropertyAtomprotocol
## DESCRIPTION


`XmDeactivateProtocol`deactivates a protocol without removing it.
It updates the handlers and the`property`if
the`shell`is realized. It is sometimes useful to allow
a protocol's state information (callback lists, and so on) to persist, even though
the client may choose to temporarily resign from the interaction.
The main use of this capability is to gray/ungray`f.send_msg`entries in the MWM system menu.
To support this capability,`protocol`is allowed to be in
one of two states: active or inactive.
If`protocol`is active and`shell`is realized,`property`contains the`protocol``Atom`.
If`protocol`is inactive,`Atom`is not present in
the`property`.

`XmDeactivateWMProtocol`is a convenience interface.
It calls`XmDeactivateProtocol`with the property value set to the atom returned by
interningWM_PROTOCOLS.

* **`shell`** 

Specifies the widget with which the protocol property is associated
* **`property`** 

Specifies the protocol property
* **`protocol`** 

Specifies the protocol atom


For a complete definition of VendorShell and its associated resources, see
&cdeman.VendorShell;.
## RELATED


&cdeman.mwm;,
&cdeman.VendorShell;,
&cdeman.XmActivateProtocol;,
&cdeman.XmDeactivateWMProtocol;, and
&cdeman.XmInternAtom;.