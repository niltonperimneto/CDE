# XmImVaSetValues
library call`XmImVaSetValues`An input manager function that updates attributes
of an input contextXmImVaSetValuesinput manager functionsXmImVaSetValues#include <Xm/XmIm.h>void`XmImVaSetValues`Widgetwidget
## DESCRIPTION


`XmImVaSetValues`updates attributes of the input context associated
with the specified widget. This function passes the attributes to`XICSetValues`. The initial call to this routine should pass in
all of the input context attributes. Thereafter, the application
programmer calls`XmImVaSetValues`only if a value has changed. See
the description in the &cdeman.XmImSetValues; man page for a list of
associated resources.

This routine uses the ANSI C variable-length
argument list (varargs) calling convention. The variable-length argument
list consists of groups of arguments each of which contains an attribute
followed by the value of the attribute. The last argument in the list must
be NULL.

Note that the List and TextField widgets call the`XmImVaSetValues`function internally. Therefore,
further calls to the`XmImVaSetValues`function for these
widgets are unnecessary.

* **`widget`** 

Specifies the ID of the widget registered with the input manager


For more information on variable-length argument lists, see
the X Toolkit Intrinsics documentation.
## RELATED


&cdeman.XmImSetFocusValues;,
&cdeman.XmImSetValues;, and
&cdeman.XmImVaSetFocusValues;.