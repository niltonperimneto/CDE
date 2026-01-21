# XmStringPeekNextComponent
library call`XmStringPeekNextComponent`A compound string function that returns the component type of the next component to be fetchedXmStringPeekNextComponentcompound string functionsXmStringPeekNextComponent#include <Xm/Xm.h>XmStringComponentType`XmStringPeekNextComponent`XmStringContextcontext
## DESCRIPTION


This function is obsolete and exists for compatibility with previous
releases. It is replaced by`XmStringPeekNextTriple`.`XmStringPeekNextComponent`examines the next component that would be fetched
by`XmStringGetNextComponent`and returns the component type.

* **`context`** 

Specifies the string context structure that was allocated by the`XmStringInitContext`function

## RETURN


Returns the type of component found.
Refer to the
&cdeman.XmStringComponentType; reference page for a list of component types.
## RELATED


&cdeman.XmStringComponentType;,
&cdeman.XmStringCreate;,
&cdeman.XmStringGetNextComponent;, and
&cdeman.XmStringInitContext;.