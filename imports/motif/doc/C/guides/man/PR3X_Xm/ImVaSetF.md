# XmImVaSetFocusValues
library call`XmImVaSetFocusValues`An input manager function that notifies an
input manager that a widget has received input focus and updates the input
context attributesXmImVaSetFocusValuesinput manager functionsXmImVaSetFocusValues#include <Xm/XmIm.h>void`XmImVaSetFocusValues`Widgetwidget
## DESCRIPTION


`XmImVaSetFocusValues`notifies the input manager that the
specified widget has received input focus. This function also updates the
attributes of the input context associated with the widget. This
function passes the attributes and values to`XICSetValues`. The
caller of this routine should pass in only those values that have changed
since the last call to any of these functions;`XmImVaSetValues`,`XmImVaSetFocusValues`,`XmImSetValues`, or`XmImSetFocusValues`.
See the description in the &cdeman.XmImSetValues; reference page for a list of
associated resources.

This routine uses the ANSI C variable-length argument list (varargs) calling
conventions. The variable-length argument list consists of groups
of arguments each of which contains an attribute followed by the
value of the attribute. The last argument in the list must be NULL

Note that the List and TextField widgets call the`XmImVaSetFocusValues`function when they receive focus. Therefore,
further calls to the`XmImVaSetFocusValues`function for these
widgets are unnecessary.

* **`widget`** 

Specifies the ID of the widget registered with the input manager


For more information on variable-length argument lists, see the
X Toolkit Intrinsics documentation.
## RELATED


&cdeman.XmImSetFocusValues;,
&cdeman.XmImSetValues;, and
&cdeman.XmImVaSetValues;.