# XmSetProtocolHooks
library call`XmSetProtocolHooks`A VendorShell function that allows preactions and postactions to be executed when a protocol message is received from MWMXmSetProtocolHooksVendorShell functionsXmSetProtocolHooksprotocols#include <Xm/Xm.h>
#include <Xm/Protocols.h>void`XmSetProtocolHooks`WidgetshellAtompropertyAtomprotocolXtCallbackProcprehookXtPointerpre_closureXtCallbackProcposthookXtPointerpost_closure
## DESCRIPTION


`XmSetProtocolHooks`is used by shells that want to have preactions
and postactions
executed when a protocol message is received from MWM.
Since there is no guaranteed ordering in execution of event handlers or
callback lists, this allows the shell to control the flow while leaving the
protocol manager structures opaque.

`XmSetWMProtocolHooks`is a convenience interface.
It calls`XmSetProtocolHooks`with the property value set to the atom returned by
interningWM_PROTOCOLS.

* **`shell`** 

Specifies the widget with which the protocol property is associated
* **`property`** 

Specifies the protocol property
* **`protocol`** 

Specifies the protocol atom
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
&cdeman.XmInternAtom;, and &cdeman.XmSetWMProtocolHooks;.