# XmImSetFocusValues
library call`XmImSetFocusValues`An input manager function that notifies an input manager that a widget
has received input focus and updates the input context attributesXmImSetFocusValuesinput manager functionsXmImSetFocusValues#include <Xm/XmIm.h>void`XmImSetFocusValues`WidgetwidgetArgListarglistCardinalargcount
## DESCRIPTION


`XmImSetFocusValues`notifies the input manager that the
specified widget has received input focus. This function also updates the
attributes of the input context associated with the widget. The focus
window for the XIC is set to the window of the widget. The`arglist`argument is a list of attribute/value pairs for the input
context. This function passes the attributes and values to`XICSetValues`. The caller of this routine should pass in
only those values that have changed since the last call to any of
these functions;`XmImSetValues`,`XmImSetFocusValues`,`XmImVaSetValues`, or`XmImVaSetFocusValues`. See the
description in the &cdeman.XmImSetValues; reference page for a list of
associated resources.

If the previous parameters for the widget's XIC do not allow
the previously registered XIC to be reused, that XIC will be
unregistered, and a new one will be created and registered with the
widget. Note that sharing of data is preserved.

* **`widget`** 

Specifies the ID of the widget registered with the input manager.
* **`arglist`** 

Specifies the list of attribute/value pairs to be passed to`XICSetValues`. See the description in the &cdeman.XmImSetValues;
man page for a description of resources.
* **`argcount`** 

Specifies the number of attribute/values pairs in the argument
list (`arglist)`


Note that the Text and TextField widgets call the`XmImSetFocusValues`function when they receive focus. Therefore,
further calls to the`XmImSetFocusValues`function for these
widgets are unnecessary.
## RELATED


&cdeman.XmImSetValues;,
&cdeman.XmImVaSetFocusValues;, and
&cdeman.XmImVaSetValues;.