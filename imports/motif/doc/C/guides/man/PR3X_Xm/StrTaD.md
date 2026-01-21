# XmStringTableToXmString
library call`XmStringTableToXmString`A convenience function that converts a compound string table to a single compound stringXmStringTableToXmString#include <Xm/Xm.h>XmString`XmStringTableToXmString`XmStringTabletableCardinalcountXmStringbreak_component
## DESCRIPTION


`XmStringTableToXmString`takes as input`table`of compound
strings and a specified
string component (such as a tab) and returns a single compound
string consisting of
each of the elements of`table`concatenated together with a
single copy of`break_component`inserted between each element.

* **`table`** 

Specifies anXmStringTablecontaining the compound strings to be
converted.
* **`count`** 

Specifies the number of compound strings in`table`.
* **`break_component`** 

Specifies the`XmStringComponent`that will be inserted in the
result to separate the individual elements of`table`. The most
useful types will be`XmSTRING_COMPONENT_SEPARATOR`and`XmSTRING_COMPONENT_TAB`.
Refer to the
&cdeman.XmStringComponentType; reference page for a complete list of
possible component types. Note, however, that the`XmSTRING_COMPONENT_UNKNOWN`component is not a possible type.

## RETURN


Returns a newXmString.
The function will allocate space to hold the returned compound string.
When the application no longer needs the returned compound string,
the application should call`XmStringFree`.
## RELATED


&cdeman.XmString;,
&cdeman.XmStringComponentType;, and
&cdeman.XmStringFree;.