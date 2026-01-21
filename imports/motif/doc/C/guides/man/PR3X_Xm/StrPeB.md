# XmStringPeekNextTriple
library call`XmStringPeekNextTriple`A function that returns the component type of the next componentXmStringPeekNextTriple#include <Xm/Xm.h>XmStringComponentType`XmStringPeekNextTriple`XmStringContextcontext
## DESCRIPTION


`XmStringPeekNextTriple`examines the next component that would be fetched
by`XmStringGetNextTriple`and returns the component type.

* **`context`** 

Specifies the string context structure that was allocated by the`XmStringInitContext`function.

## RETURN


Returns the type of the component found.
Refer to the
&cdeman.XmStringComponentType; reference page for a list of component types.
## RELATED


&cdeman.XmString;,
&cdeman.XmStringComponentType;, and
&cdeman.XmStringGetNextTriple;.