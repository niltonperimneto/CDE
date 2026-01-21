# XmSetWMProtocolHooks
library call`XmSetWMProtocolHooks`A VendorShell convenience interface that allows preactions and postactions to be executed when a protocol message is received from the window managerXmSetWMProtocolHooksVendorShell functionsXmSetWMProtocolHooksprotocols#include <Xm/Xm.h>
#include <Xm/Protocols.h>void`XmSetWMProtocolHooks`WidgetshellAtomprotocolXtCallbackProcprehookXtPointerpre_closureXtCallbackProcposthookXtPointerpost_closure
## DESCRIPTION


`XmSetWMProtocolHooks`is a convenience interface.
It calls`XmSetProtocolHooks`with the property value set to the atom returned by
interningWM_PROTOCOLS.

* **`shell`** 

Specifies the widget with which the protocol property is associated
* **`protocol`** 

Specifies the protocol atom (or an`int`cast to`Atom`)
* **`prehook`** 

Specifies the procedure to call before calling entries on the client callback
list
* **`pre_closure`** 

Specifies the client data to be passed to the prehook when it is invoked
* **`posthook`** 

Specifies the procedure to call after calling entries on the client callback
list
* **`post_closure`** 

Specifies the client data to be passed to the posthook when it is invoked


For a complete definition of VendorShell and its associated resources, see
&cdeman.VendorShell;.
## RELATED


&cdeman.VendorShell;,
&cdeman.XmInternAtom;, and &cdeman.XmSetProtocolHooks;.