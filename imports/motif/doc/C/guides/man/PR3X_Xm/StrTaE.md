# XmStringTableUnparse
library call`XmStringTableUnparse`A convenience function that converts a table of compound strings to an array of textXmStringTableUnparse#include <Xm/Xm.h>XtPointer *`XmStringTableUnparse`XmStringTabletableCardinalcountXmStringTagtagXmTextTypetag_typeXmTextTypeoutput_typeXmParseTableparseCardinalparse_countXmParseModelparse_model
## DESCRIPTION


`XmStringTableUnparse`takes an array of compound strings,
allocates a string array for the type of characters determined by`type`with an equal number of slots, calls`XmStringUnparse`on each compound string in`table`,
and inserts the resulting string in the corresponding slot in the array.

* **`table`** 

Specifies anXmStringTablecontaining the compound string to be
converted.
* **`count`** 

Specifies the number of compound strings in`table`.
* **tag** 

Specifies the tag to be used in matching with text segments.
The two
types of tag types are`XmFONTLIST_DEFAULT_TAG`and`_MOTIF_DEFAULT_LOCALE`.
Only segments tagged withtagwill be returned. Iftagis NULL, all segments will be matched.
* **`tag_type`** 

Specifies the type of tag to be searched for. These types include`XmMULTIBYTE_TEXT`,`XmWIDECHAR_TEXT`, and`XmCHARSET_TEXT`.
* **`output_type`** 

Specifies the type of text to be generated. These types include`XmMULTIBYTE_TEXT`,`XmWIDECHAR_TEXT`, and`XmCHARSET_TEXT`.
* **`parse`** 

Specifies the parse table to be used.
* **`parse_count`** 

Specifies the number of items in`parse`.
* **`parse_model`** 

Specifies which non-text components to be considered in matching in`parse_table`. Possible values are:

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

Puts out only those matching components that are at the beginning or
end of a
matching text component.


## RETURN


Returns an allocated array of allocated strings.
The application is responsible for managing the allocated space.
The application can recover the allocated strings space by calling`XtFree``count`times (that is, one time for each allocated string).
The application can then recover the allocated array by calling`XtFree`on the allocated array itself.
## RELATED


`XmStringTab.`