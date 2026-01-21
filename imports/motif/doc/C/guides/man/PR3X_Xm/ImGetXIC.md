# XmImGetXIC
library call`XmImGetXIC`An input manager function that obtains an XIC for a widgetXmImGetXICinput manager functionsXmImGetXIC#include <Xm/XmIm.h>XIC`XmImGetXIC`WidgetwidgetXmInputPolicyinput_policyArgListargsCardinalnum_args
## DESCRIPTION


`XmImGetXIC`creates and registers an X Input Context (XIC)
with the specified
arguments for`widget`. If`XmINHERIT_POLICY`is specified
for`input_policy`, a new XIC will be created only if
required to by the arguments or by the`VendorShell`input policy.
Any existing XIC registered with`widget`is unregistered.

Refer to the`VendorShell`reference page for further details.

* **`widget`** 

Specifies the ID of a widget for which an Input Context is to be
created.
* **`input_policy`** 

Specifies the type of input policy. It accepts the following values:

* **`XmINHERIT_POLICY`** 

Inherits the policy from`VendorShell`.
* **`XmPER_WIDGET`** 

Creates a new XIC for this widget.
* **`XmPER_SHELL`** 

Creates a new XIC for the shell, if needed.

* **`args`** 

Specifies an`XtArgList`parameter to use for creating the XIC.
* **`num_args`** 

Specifies the number of arguments in the`args`parameter.

## RETURN


Returns the created XIC.
The application is responsible for freeing the returned XIC
by calling`XmImFreeXIC`.
## RELATED


&cdeman.XmImSetXIC; and
&cdeman.XmImFreeXIC;.