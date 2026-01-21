# XmParseMappingCreate
library call`XmParseMappingCreate`A compound string function to create a parse mappingXmParseMappingCreatecompound string functionsXmParseMappingCreate#include <Xm/Xm.h>XmParseMapping`XmParseMappingCreate`ArgListarglistCardinalargcount
## DESCRIPTION


`XmParseMappingCreate`creates a parse mapping for use in a parse
table.
This function allows the application to specify values for components of
the parse mapping using a resource-style argument list.

* **`arglist`** 

Specifies the argument list
* **`argcount`** 

Specifies the number of attribute/value pairs in the argument list
(`arglist`)


For a complete definition ofXmParseMappingand its associated
resources, see &cdeman.XmParseMapping;.
## RETURN


Returns theXmParseMappingobject.
The function allocates space to hold the returnedXmParseMappingobject.
The application is responsible for managing the allocated space.
The application can recover the allocated space by calling`XmParseMappingFree`.
## RELATED


&cdeman.XmParseMapping;,
&cdeman.XmParseMappingFree;,
&cdeman.XmParseMappingGetValues;,
&cdeman.XmParseMappingSetValues;,
&cdeman.XmParseTable;, and
&cdeman.XmString;.