# XmStringGetNextTriple
library call`XmStringGetNextTriple`An XmString function that returns the type, length, and value of the next component in the compound stringXmStringGetNextTriple#include <Xm/Xm.h>XmStringComponentType`XmStringGetNextTriple`XmStringContextcontextunsigned int*lengthXtPointer*value
## DESCRIPTION


`XmStringGetNextTriple`returns the type, length, and value of the
next component in the compound string identified by`context`.
This function returns one component at a time.

* **`context`** 

Specifies the string context structure that was allocated by the`XmStringInitContext`function.
* **`length`** 

Specifies a pointer to the length of the value of the returned component.
* **`value`** 

Specifies a pointer to the value of the returned component.
If the returned value is not NULL, the function allocates space to hold
the returned value.
When the application no longer needs the returned compound string,
the application should call`XtFree`.

## RETURN


Returns the type of the component found. Refer to the
&cdeman.XmStringComponentType; reference page for a list of component types.
## RELATED


&cdeman.XmDirection;,
&cdeman.XmString;,
&cdeman.XmStringComponentType;,
&cdeman.XmStringGetNextComponent;, and
&cdeman.XmStringPeekNextTriple;.