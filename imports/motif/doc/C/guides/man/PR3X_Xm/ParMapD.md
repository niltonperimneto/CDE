# XmParseMappingGetValues
library call`XmParseMappingGetValues`A compound string function to retrieve attributes of a parse mappingXmParseMappingGetValuescompound string functionsXmParseMappingGetValues#include <Xm/Xm.h>void`XmParseMappingGetValues`XmParseMappingparse_mappingArgListarglistCardinalargcount
## DESCRIPTION


`XmParseMappingGetValues`retrieves attributes of anXmParseMappingobject, using a resource-style argument list.
If the`XmNsubstitute`resource is in the`arglist`, the
function will allocate space to hold the returnedXmStringvalue.
The application is responsible for managing this allocated space.
The application can recover the allocated space by calling`XmStringFree`.

* **`arglist`** 

Specifies the argument list
* **`argcount`** 

Specifies the number of attribute/value pairs in the argument list
(`arglist`)


For a complete definition ofXmParseMappingand its associated
resources, see &cdeman.XmParseMapping;.
## RELATED


&cdeman.XmParseMapping;,
&cdeman.XmParseMappingCreate;,
&cdeman.XmParseMappingFree;,
&cdeman.XmParseMappingSetValues;,
&cdeman.XmParseTable;, and
&cdeman.XmString;.