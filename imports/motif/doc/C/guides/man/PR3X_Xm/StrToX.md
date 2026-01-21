# XmStringToXmStringTable
library call`XmStringToXmStringTable`A convenience function that converts a single compound string to a table of compound stringsXmStringTableToXmString#include <Xm/Xm.h>Cardinal`XmStringToXmStringTable`XmStringstringXmStringbreak_componentXmStringTable*table
## DESCRIPTION


`XmStringToXmStringTable`takes as input a single compound string
and a specified
string component (such as a tab) and returns a table of compound
strings consisting of portions of`string`delimited by components
matching`break_component`. The components marking breaks will
not appear in the resulting table.

* **`string`** 

Specifies theXmStringto be converted.
* **`break_component`** 

Specifies the`XmStringComponent`that will be used to indicate
where to split`string`to form the individual elements of`table`. The most
useful types will be`XmSTRING_COMPONENT_SEPARATOR`and`XmSTRING_COMPONENT_TAB`. Refer to the
&cdeman.XmStringComponentType;
reference page for a complete list of
possible component types. Note, however, that the`XmSTRING_COMPONENT_UNKNOWN`component is not a possible type.
* **`table`** 

Returns the equivalentXmStringTable.
The function will allocate space to hold the returnedXmStringTable.
When the applicaiton no longer needs the returnedXmStringTable,
the application should call`XmStringFree`once for each compound string in the table,
and then calling`XtFree`to deallocate theXmStringTableitself.

## RETURN


Returns the number of compound strings in`table`.
## RELATED


&cdeman.XmStringTable;.