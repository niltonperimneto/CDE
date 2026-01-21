# XmStringTableParseStringArray
library call`XmStringTableParseStringArray`A convenience function that converts an array of strings to a compound string tableXmStringTableParseStringArray#include <Xm/Xm.h>XmStringTable`XmStringTableParseStringArray`XtPointer*stringsCardinalcountXmStringTagtagXmTextTypetypeXmParseTableparseCardinalparse_countXtPointercall_data
## DESCRIPTION


`XmStringTableParseStringArray`takes an array of strings,
allocates anXmStringTablewith an equal number of slots, calls`XmStringParseText`on each string in`strings`, and inserts
the resultingXmStringin the corresponding slot in theXmStringTable.

* **`strings`** 

Specifies an array of strings of characters as determined by`type`.
* **`count`** 

Specifies the number of strings in`strings`.
* **tag** 

Specifies the tag to be used in creating the result. The type of tag
created (charset or locale) depends on the type of the text and
the value given. If the value specified is NULL, and`type`indicates that a charset tag should be created, then the tag will
have the value of`XmFONTLIST_DEFAULT_TAG`.
If`type`indicates a locale tag, then the tag will have the value of`XmFONTLIST_DEFAULT_TAG`.
* **`type`** 

Specifies the type of text to be passed in and the type of tag. If
the type is either`XmMULTIBYTE_TEXT`or`XmWIDECHAR_TEXT`, a
locale tag should be created. If the type is`XmCHARSET_TEXT`, a
charset tag
will
be created.
* **`parse`** 

Specifies the parse table to be used.
* **`parse_count`** 

Specifies the number of entries in the parse table.
* **`call_data`** 

Specifies data to be passed to the parse procedures.

## RETURN


Returns a newXmStringTable.
The function allocates space to hold theXmStringTable.
When the application no longer needs the returnedXmStringTable,
the application should call`XmStringFree``count`times (that is, one time for each returned compound string)
and then call`XtFree`to deallocate theXmStringTableitself.
## RELATED


&cdeman.XmStringFree; and
&cdeman.XmTabList;.