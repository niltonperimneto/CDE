# XmStringByteCompare
library call`XmStringByteCompare`A compound string function that indicates the results of a byte-by-byte comparisonXmStringByteComparecompound string functionsXmStringByteCompare#include <Xm/Xm.h>Boolean`XmStringByteCompare`XmStrings1XmStrings2
## DESCRIPTION


This function is obsolete and exists for compatibility with previous
releases.`XmStringByteCompare`returns a Boolean indicating the results of a byte-by-byte
comparison of two compound strings.

In general, if two compound strings are created with the same (char *)
string using`XmStringCreateLocalized`in the same language
environment, the compound strings compare as equal.
If two compound strings are created with the same (`char&ensp;*`) string and
the same font list element tag set other than`XmFONTLIST_DEFAULT_TAG`using`XmStringCreate`, the strings compare as
equal.

In some cases, once a compound string
is put into a widget, that string is converted into an internal form to
allow faster processing. Part of the conversion process strips out
unnecessary or redundant information. If an application then does an`XtGetValues`to retrieve a compound string from a widget (specifically,
Label and all of its subclasses), it is not guaranteed that the compound
string returned is byte-for-byte the same as the string given to
the widget originally.

* **`s1`** 

Specifies a compound string to be compared with`s2`
* **`s2`** 

Specifies a compound string to be compared with`s1`

## RETURN


Returns True if two compound strings are identical byte-by-byte.
## RELATED


&cdeman.XmStringCreate; and
&cdeman.XmStringCreateLocalized;.