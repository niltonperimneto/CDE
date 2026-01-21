# XmStringUnparse
library call`XmStringUnparse`A compound string function that unparses textXmStringUnparse#include <Xm/Xm.h>XtPointer`XmStringUnparse`XmStringstringXmStringTagtagXmTextTypetag_typeXmTextTypeoutput_typeXmParseTableparse_tableCardinalparse_countXmParseModelparse_model
## DESCRIPTION


`XmStringUnparse`looks in the input`string`for text
segments that are
tagged with locale or charset tags that matchtag. The`tag_type`parameter specifies whether the tag is a locale or charset type.
Iftaghas a value of NULL, all the segments are matched. When
a text segment is found with a matching tag, it is added to the end of
a resulting string. The characters in the
resulting string are of type`output_type`.

`XmStringUnparse`also checks`string`for components that
match components in`parse_table`, and also to see if the
component matches the condition specified by`parse_model`. If
the string component matches in both checks, then the associated
character is added to the end of the resulting string.

* **`string`** 

Specifies theXmStringto be converted.
* **tag** 

Specifies the tag to be used in matching with text segments. Only text
segments that matchtagwill be included in the resulting
string. Iftaghas a value of NULL, all segments are considered
as
matches, and`tag_type`is ignored.
* **`tag_type`** 

Specifies the type of tag to be searched for, including`XmMULTIBYTE_TEXT`,`XmWIDECHAR_TEXT`, and`XmCHARSET_TEXT`.
* **`output_type`** 

Specifies the type of text to be returned in the string, including`XmMULTIBYTE_TEXT`,`XmWIDECHAR_TEXT`, and`XmCHARSET_TEXT`.
* **`parse_table`** 

Specifies the parse table to be used in scanning for compound string
components to be
converted to other characters.
* **`parse_count`** 

Specifies how many entries are in`parse_table`.
* **`parse_model`** 

Specifies which non-text components to be considered in matching in`parse_table`. These include:

* **`XmOUTPUT_ALL`** 

Puts out all matching components.
* **`XmOUTPUT_BETWEEN`** 

Puts out only those matching components that are between two matching
text components.
* **`XmOUTPUT_BEGINNING`** 

Puts out only those matching components that are at the beginning of a
matching text component.
* **`XmOUTPUT_END`** 

Puts out only those matching components that are at the end of a
matching text component.
* **`XmOUTPUT_BOTH`** 

Puts out only those matching components that are at the beginning or end
of a matching text component.


## RETURN


Returns a newly allocated string containing characters of a type
determined by`output_type`.
The application is responsible for managing this allocated space.
The application can recover this allocated space by calling`XtFree`.
## RELATED


&cdeman.XmString;, &cdeman.XmParseTable;, &cdeman.XmParseMapping;.